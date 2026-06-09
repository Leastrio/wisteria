use gpui::{Context, IntoElement, Render, Window};

use crate::widgets::*;

pub struct WorkspacesWidget;

impl Render for WorkspacesWidget {
  fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    pill()
      .gap_1p5()
      .h_full()
      .px_0p5()
      .child(workspace_button(false, 1))
      .child(workspace_button(false, 2))
      .child(workspace_button(true, 3))
      .child(workspace_button(false, 4))
      .child(workspace_button(false, 5))
  }
}
