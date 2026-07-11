pub mod compositor;
pub mod mpris;
pub mod sys_info;

use std::{
  any::{Any, TypeId},
  collections::HashMap,
  env,
  sync::Arc,
};

use gpui::{App, Global};
use tracing::info;
use zbus::Connection;

use crate::compositor::{CompositorService, hyprland::HyprlandService};

pub trait Service: Send + Sync + 'static {
  fn new() -> Arc<Self>;
  fn start(self: Arc<Self>, cx: &mut App);
}

pub struct ServiceRegistry {
  services: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
  session: Connection,
}

impl Global for ServiceRegistry {}

impl ServiceRegistry {
  pub async fn new() -> Self {
    info!("Opening session bus");
    Self {
      services: HashMap::new(),
      session: Connection::session().await.unwrap(),
    }
  }

  pub fn service<T>(&mut self, cx: &mut App) -> Arc<T>
  where
    T: Service,
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

  pub fn compositor(&mut self, cx: &mut App) -> Option<Arc<dyn CompositorService>> {
    match env::var("XDG_CURRENT_DESKTOP")
      .ok()
      .or_else(|| env::var("DESKTOP_SESSION").ok())
      .as_deref()
    {
      Some("Hyprland") => Some(self.service::<HyprlandService>(cx)),
      _ => None,
    }
  }

  // Id perfer to have lazy initialization however couldnt get it to work todo later
  pub fn session(&self) -> Connection {
    self.session.clone()
  }
}
