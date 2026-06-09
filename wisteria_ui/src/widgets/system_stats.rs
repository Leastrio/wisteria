use gpui::{Context, IntoElement, Render, Window};

use crate::widgets::*;

pub struct SysStatsWidget;

impl Render for SysStatsWidget {
  fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    pill()
      .child(svg().path("icons/cpu_usage.svg").text_color(rgb(0xcdd6f4)).size_4().mr_1())
      .child("2% ")
      .child(svg().path("icons/ram.svg").text_color(rgb(0xcdd6f4)).size_4().mr_1())
      .child("12G ")
      .child(svg().path("icons/drive.svg").text_color(rgb(0xcdd6f4)).size_4().mr_1())
      .child("44%")
  }
}
