use std::{env, error::Error, os::unix::net::UnixStream, pin::Pin, sync::Arc};

use async_io::Async;
use futures_lite::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, io::BufReader};
use futures_signals::signal::{Mutable, MutableSignalCloned};
use gpui::AppContext;
use serde_json::Value;
use tracing::info;

use crate::{
  Service,
  compositor::{CompositorService, Workspace, Workspaces},
};

pub struct HyprlandService {
  workspaces: Mutable<Workspaces>,
  his: String,
}

impl Service for HyprlandService {
  fn new() -> Arc<Self> {
    Arc::new(Self {
      workspaces: Mutable::new(Vec::new()),
      his: env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap(),
    })
  }

  fn start(self: Arc<Self>, cx: &mut gpui::App) {
    cx.background_spawn(async move {
      let _ = self.run().await;
    })
    .detach();
  }
}

impl CompositorService for HyprlandService {
  fn subscribe(&self) -> MutableSignalCloned<Workspaces> {
    self.workspaces.signal_cloned()
  }

  fn select_workspace(
    self: Arc<Self>,
    display_name: String,
    workspace_id: u64,
  ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    Box::pin(async move {
      let _ = self
        .send_msg(&format!(
          "dispatch hl.dsp.focus({{ workspace = {0} }})",
          workspace_id
        ))
        .await;
      self.set_active_workspace(&display_name, workspace_id);
    })
  }
}

impl HyprlandService {
  pub async fn run(self: Arc<Self>) -> std::result::Result<(), Box<dyn Error>> {
    self.get_workspaces().await?;
    self.get_active_workspace().await?;

    let xdg_runtime_dir = env::var("XDG_RUNTIME_DIR")?;
    let path = format!("{}/hypr/{}/.socket2.sock", xdg_runtime_dir, self.his);
    let stream = Async::new(UnixStream::connect(path)?)?;
    let mut reader = BufReader::new(stream);

    let mut created_workspace_id: Option<u64> = None;

    loop {
      let mut line = String::new();
      let n = reader.read_line(&mut line).await?;
      if n == 0 {
        info!("Hyprland socket closed");
        break;
      }

      let parts: Vec<&str> = line.trim().split(">>").collect();
      match parts[0] {
        "focusedmonv2" => {
          let data: Vec<&str> = parts[1].split(",").collect();
          self.set_active_workspace(data[0], data[1].parse::<u64>()?);
          self.maybe_set_display(data[0], created_workspace_id);
          created_workspace_id = None;
        }
        "workspacev2" => {
          let data: Vec<&str> = parts[1].split(",").collect();
          self.set_active_workspace("", data[0].parse::<u64>()?);
        }
        "createworkspacev2" => {
          let data: Vec<&str> = parts[1].split(",").collect();
          let id: u64 = data[0].parse()?;
          created_workspace_id = Some(id);
          self.workspaces.lock_mut().push(Workspace {
            display: String::new(),
            id: id,
            name: data[1].to_owned(),
            active: false,
          });
        }
        "destroyworkspacev2" => {
          let data: Vec<&str> = parts[1].split(",").collect();
          let id = data[0].parse::<u64>()?;
          let mut workspaces = self.workspaces.lock_mut();
          if let Some(pos) = workspaces.iter().position(|w| w.id == id) {
            workspaces.remove(pos);
          }
        }
        _ => {
          println!("{:#?}", parts);
        }
      }
    }

    Ok(())
  }

  async fn get_workspaces(&self) -> Result<(), Box<dyn Error>> {
    let resp = self.send_msg("j/workspaces").await?;
    let val: Value = serde_json::from_str(&resp)?;
    let mut workspaces = self.workspaces.lock_mut();

    for workspace in val.as_array().ok_or("No array")? {
      workspaces.push(Workspace {
        display: workspace["monitor"].as_str().unwrap().to_owned(),
        id: workspace["id"].as_u64().unwrap(),
        name: workspace["name"].as_str().unwrap().to_owned(),
        active: false,
      });
    }

    Ok(())
  }

  async fn get_active_workspace(&self) -> Result<(), Box<dyn Error>> {
    let resp = self.send_msg("j/activeworkspace").await?;
    let val: Value = serde_json::from_str(&resp)?;

    let display_name = val["monitor"].as_str().unwrap();
    let workspace_id = val["id"].as_u64().unwrap();
    self.set_active_workspace(display_name, workspace_id);

    Ok(())
  }

  fn set_active_workspace(&self, display_name: &str, id: u64) {
    let mut workspaces = self.workspaces.lock_mut();
    for workspace in workspaces.iter_mut() {
      if workspace.id == id {
        workspace.active = true;

        if workspace.display.is_empty() {
          workspace.display = display_name.to_owned();
        }
      } else {
        workspace.active = false;
      }
    }
  }

  fn maybe_set_display(&self, display_name: &str, id: Option<u64>) {
    if id.is_none() {
      return;
    }

    let mut workspaces = self.workspaces.lock_mut();

    let item = workspaces.iter_mut().find(|w| w.id == id.unwrap()).unwrap();

    if item.display.is_empty() {
      item.display = display_name.to_owned();
    }
  }

  async fn send_msg(&self, msg: &str) -> Result<String, Box<dyn Error>> {
    let xdg_runtime_dir = env::var("XDG_RUNTIME_DIR")?;
    let path = format!("{}/hypr/{}/.socket.sock", xdg_runtime_dir, self.his);
    let mut resp = String::new();

    let mut stream = Async::new(UnixStream::connect(path)?)?;
    stream.write_all(msg.as_bytes()).await?;
    stream.read_to_string(&mut resp).await?;
    return Ok(resp);
  }
}
