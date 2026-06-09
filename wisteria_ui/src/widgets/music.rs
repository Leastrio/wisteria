use gpui::{Context, IntoElement, Render, Window, img};

use crate::widgets::*;

pub struct MusicWidget;

impl Render for MusicWidget {
  fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    pill()
      .child(
        img("https://i.scdn.co/image/ab67616d0000b273ffcea832441929c024df21e2")
          .size_5()
          .rounded_full()
          .overflow_hidden()
          .border_1()
          .border_color(rgb(0x89b4fa))
          .mr_1()
      )
      .child("Luna Li - That's Life")
  }
}
