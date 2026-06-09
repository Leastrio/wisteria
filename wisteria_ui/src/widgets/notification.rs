use gpui::{Context, IntoElement, Render, Window};

use crate::widgets::*;

pub struct NotificationWidget;

impl Render for NotificationWidget {
  fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    icon_button("icons/notification.svg")
  }
}
