use gpui::{Context, IntoElement, Render, Window};

use crate::widgets::*;

pub struct LauncherWidget;

impl Render for LauncherWidget {
  fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    icon_button("icons/launcher.svg")
  }
}
