pub mod music;

use std::sync::Arc;

use gpui::{App, Global};

use crate::music::MprisService;

#[derive(Default)]
pub struct ServiceRegistry {
  pub mpris: Option<Arc<MprisService>>
}

impl Global for ServiceRegistry {}

impl ServiceRegistry {
  pub fn mpris(&mut self, cx: &mut App) -> Arc<MprisService> {
    if let Some(service) = &self.mpris {
      return service.clone();
    }

    let service = MprisService::new();
    let task = service.clone();
    cx.spawn(async move |_| {
      task.run().await;
    }).detach();

    self.mpris = Some(service.clone());
    
    service
  }
}
