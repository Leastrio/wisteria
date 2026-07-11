use std::time::Duration;

use chrono::Local;
use gpui::{Context, IntoElement, Render, Window};

use crate::widgets::*;

pub struct ClockWidget {
  text: String,
}

impl ClockWidget {
  pub fn new(cx: &mut Context<Self>) -> Self {
    cx.spawn(async move |this, cx| {
      loop {
        let now = Local::now().format("%l:%M %p • %a, %b %d").to_string();

        let _ = this.update(cx, |view, cx| {
          view.text = now.trim().to_owned();
          cx.notify();
        });

        cx.background_executor()
          .timer(Duration::from_millis(1000))
          .await;
      }
    })
    .detach();

    ClockWidget {
      text: String::new(),
    }
  }
}

impl Render for ClockWidget {
  fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    pill().child(self.text.clone())
  }
}
