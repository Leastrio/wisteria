pub mod mpris;
pub mod sys_info;

use std::{any::{Any, TypeId}, collections::HashMap, sync::Arc};

use gpui::{App, Global};
use tracing::info;

pub trait Service: Send + Sync + 'static {
  fn new() -> Arc<Self>;
  fn start(self: Arc<Self>, cx: &mut App);
}

#[derive(Default)]
pub struct ServiceRegistry {
  services: HashMap<TypeId, Arc<dyn Any + Send + Sync>>
}

impl Global for ServiceRegistry {}

impl ServiceRegistry {
  pub fn service<T>(&mut self, cx: &mut App) -> Arc<T>
  where 
    T: Service
  {
    let id = TypeId::of::<T>();

    if let Some(existing) = self.services.get(&id) {
      return existing.clone().downcast::<T>().unwrap();
    }

    info!("Spawning new service: {}", std::any::type_name::<T>());

    let service = T::new();
    service.clone().start(cx);
    self.services.insert(id, service.clone());
    service
  }
}
