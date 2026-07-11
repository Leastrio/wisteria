use std::sync::Arc;

use futures_signals::signal::SignalExt;
use futures_util::StreamExt;
use gpui::{Context, IntoElement, Render, Window};
use wisteria_services::{
  ServiceRegistry,
  compositor::{CompositorService, Workspace, Workspaces},
};

use crate::widgets::*;

pub struct WorkspacesWidget {
  display_name: String,
  service: Option<Arc<dyn CompositorService>>,
  workspaces: Workspaces,
}

impl WorkspacesWidget {
  pub fn new(cx: &mut Context<Self>, display_name: String) -> Self {
    let service = cx.update_global::<ServiceRegistry, _>(|registry, cx| registry.compositor(cx));

    if let Some(service) = &service {
      let mut stream = service.subscribe().to_stream();

      cx.spawn(async move |this, cx| {
        while let Some(workspaces) = stream.next().await {
          let _ = this.update(cx, |this, cx| {
            this.workspaces = workspaces;
            println!("{:#?}", this.workspaces);
            cx.notify();
          });
        }
      })
      .detach();
    }

    Self {
      display_name,
      service,
      workspaces: Vec::new(),
    }
  }
}

impl Render for WorkspacesWidget {
  fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    let mut workspaces: Vec<&Workspace> = self
      .workspaces
      .iter()
      .filter(|w| w.display == self.display_name)
      .collect();

    workspaces.sort_by(|a, b| a.id.cmp(&b.id));
    if let Some(service) = &self.service
      && !workspaces.is_empty()
    {
      let service = service.clone();

      let mut buttons: Vec<Div> = Vec::new();
      workspaces.iter().for_each(|w| {
        buttons.push(workspace_button(&service, *w));
      });

      pill().gap_1p5().h_full().px_0p5().children(buttons)
    } else {
      div()
    }
  }
}

pub fn workspace_button(service: &Arc<dyn CompositorService>, workspace: &Workspace) -> Div {
  let service = service.clone();
  let workspace_id = workspace.id;
  let display_name = workspace.display.clone();

  div()
    .size_6()
    .rounded_full()
    .flex()
    .items_center()
    .justify_center()
    .font_weight(FontWeight::SEMIBOLD)
    .bg(if workspace.active {
      rgb(0xcba6f7)
    } else {
      rgb(0x45475a)
    })
    .text_color(if workspace.active {
      rgb(0x1e1e2e)
    } else {
      rgb(0xcdd6f4)
    })
    .child(workspace.name.clone())
    .on_mouse_down(gpui::MouseButton::Left, move |_, _, cx| {
      let service = service.clone();
      let display_name = display_name.clone();

      cx.spawn(async move |_| {
        service.select_workspace(display_name, workspace_id).await;
      })
      .detach();
    })
    .cursor_pointer()
}
