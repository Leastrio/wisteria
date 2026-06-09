use std::time::Duration;

use chrono::Local;
use gpui::{Context, IntoElement, Render, Window};

use crate::widgets::*;

pub struct ClockWidget;

impl ClockWidget {
    pub fn new(cx: &mut Context<Self>) -> Self {
      cx.spawn(async move |this, cx| {
        loop {
          let _ = this.update(cx, |_, cx| cx.notify());
          cx.background_executor()
            .timer(Duration::from_millis(1000))
            .await;
        }
      })
      .detach();

      ClockWidget
    }
}

impl Render for ClockWidget {
  fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    let now = Local::now();
    pill().child(now.format("%l:%M %p • %a, %b %d").to_string().trim().to_string())
  }
}
