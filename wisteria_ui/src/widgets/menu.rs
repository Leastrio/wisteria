use gpui::{Context, IntoElement, Render, Window};

use crate::widgets::*;

pub struct MenuWidget;

impl Render for MenuWidget {
  fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    icon_button("icons/nixos.svg")
  }
}
