use std::{path::Path, sync::Arc, time::Duration};

use futures_signals::signal::{Mutable, MutableSignalCloned};
use gpui::BackgroundExecutor;
use sysinfo::{Components, Disks, System};

use crate::Service;

#[derive(Clone, Default, Debug)]
pub struct SysStats {
  pub cpu_usage: f32,
  pub cpu_temp: f32,
  pub coolant_temp: f32,
  pub ram_used: f32,
  pub disk_usage: f32,
}

pub struct SysInfoService {
  pub stats: Mutable<SysStats>,
}

impl Service for SysInfoService {
  fn new() -> Arc<Self> {
    Arc::new(Self {
      stats: Mutable::new(SysStats::default()),
    })
  }

  fn start(self: Arc<Self>, cx: &mut gpui::App) {
    let executor = cx.background_executor().clone();
    executor
      .clone()
      .spawn(async move {
        self.run(executor).await;
      })
      .detach();
  }
}

impl SysInfoService {
  pub async fn run(self: Arc<Self>, executor: BackgroundExecutor) {
    let mut sys = System::new();
    let mut components = Components::new_with_refreshed_list();
    let mut disks = Disks::new_with_refreshed_list();

    loop {
      sys.refresh_cpu_all();
      sys.refresh_memory();
      components.refresh(false);
      disks.refresh(false);

      let cpu_usage = sys.global_cpu_usage();
      let cpu_temp = components
        .iter()
        .find(|c| c.label() == "k10temp Tctl")
        .and_then(|c| c.temperature())
        .unwrap_or(0.0);
      let coolant_temp = components
        .iter()
        .find(|c| c.label() == "d5next Coolant temp")
        .and_then(|c| c.temperature())
        .unwrap_or(0.0);
      let ram_used = sys.used_memory() as f32 / 1024.0 / 1024.0 / 1024.0;
      let disk_usage = disks
        .iter()
        .find(|d| d.mount_point() == Path::new("/"))
        .map(|d| {
          let total = d.total_space();
          let used = total - d.available_space();

          used as f32 / total as f32 * 100.0
        })
        .unwrap_or(0.0);

      self.stats.set(SysStats {
        cpu_usage,
        cpu_temp,
        coolant_temp,
        ram_used,
        disk_usage,
      });

      executor.timer(Duration::from_millis(2000)).await;
    }
  }

  pub fn subscribe(&self) -> MutableSignalCloned<SysStats> {
    self.stats.signal_cloned()
  }
}
