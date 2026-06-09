use gpui::{Context, IntoElement, Render, Window};

use crate::widgets::*;

pub struct VolumeWidget;

impl Render for VolumeWidget {
  fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    pill()
      .child(svg().path("icons/volume.svg").text_color(rgb(0xcdd6f4)).size_4().mr_1())
      .child("100%")
  }
}
