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

pub fn workspace_button(active: bool, num: usize) -> Div {
    div()
        .size_6()
        .rounded_full()
        .flex()
        .items_center()
        .justify_center()
        .font_weight(FontWeight::SEMIBOLD)
        .bg(if active {
            rgb(0xcba6f7)
        } else {
            rgb(0x45475a)
        })
        .text_color(if active {
            rgb(0x1e1e2e)
        } else {
            rgb(0xcdd6f4)
        })
        .child(num.to_string())
}
