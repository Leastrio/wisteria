use std::{pin::Pin, sync::Arc};

use futures_signals::signal::MutableSignalCloned;

pub mod hyprland;

pub type Workspaces = Vec<Workspace>;

#[derive(Clone, Debug)]
pub struct Workspace {
  pub display: String,
  pub id: u64,
  pub name: String,
  pub active: bool,
}

pub trait CompositorService: Send + Sync {
  fn subscribe(&self) -> MutableSignalCloned<Workspaces>;

  fn select_workspace(
    self: Arc<Self>,
    display_name: String,
    workspace_id: u64,
  ) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}
