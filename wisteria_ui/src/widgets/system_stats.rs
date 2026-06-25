use std::sync::Arc;

use futures_signals::signal::SignalExt;
use futures_util::StreamExt;
use gpui::{Context, IntoElement, Render, Window};
use wisteria_services::{ServiceRegistry, sys_info::{SysInfoService, SysStats}};

use crate::widgets::*;

pub struct SysStatsWidget {
  stats: SysStats,
  _service: Arc<SysInfoService>
}

impl SysStatsWidget {
  pub fn new(cx: &mut Context<Self>) -> Self {
    let service = cx.update_global::<ServiceRegistry, _>(|registry, cx| {
      registry.service::<SysInfoService>(cx)
    });

    let mut stats_stream = service.subscribe().to_stream();

    cx.spawn(async move |this, cx| {
      while let Some(stats) = stats_stream.next().await {
        let _ = this.update(cx, |this, cx| {
          this.stats = stats;
          cx.notify();
        });
      }
    }).detach();

    Self {
      stats: SysStats::default(),
      _service: service
    }
  }
}

impl Render for SysStatsWidget {
  fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    pill()
      .child(svg().path("icons/cpu_usage.svg").text_color(rgb(0xcdd6f4)).size_4().mr_1())
      .child(format!("{:.0}% ", self.stats.cpu_usage))
      .child(svg().path("icons/cpu_temp.svg").text_color(rgb(0xcdd6f4)).size_4().mr_1())
      .child(format!("{:.0}°C ", self.stats.cpu_temp))
      .child(svg().path("icons/coolant.svg").text_color(rgb(0xcdd6f4)).size_4().mr_1())
      .child(format!("{:.0}°C ", self.stats.coolant_temp))
      .child(svg().path("icons/ram.svg").text_color(rgb(0xcdd6f4)).size_4().mr_1())
      .child(format!("{}G ", self.stats.ram_used))
      .child(svg().path("icons/drive.svg").text_color(rgb(0xcdd6f4)).size_4().mr_1())
      .child(format!("{:.0}%", self.stats.disk_usage))
  }
}
