use std::sync::Arc;

use futures_signals::signal::SignalExt;
use futures_util::StreamExt;
use gpui::{Context, IntoElement, Render, Window, img};
use wisteria_services::{ServiceRegistry, mpris::{MprisService, TrackInfo}};

use crate::widgets::*;

pub struct MprisWidget {
  track: TrackInfo,
  service: Arc<MprisService>
}

impl MprisWidget {
  pub fn new(cx: &mut Context<Self>) -> Self {
    let service = cx.update_global::<ServiceRegistry, _>(|registry, cx| {
      registry.service::<MprisService>(cx)
    });

    let mut track_stream = service.subscribe().to_stream();

    cx.spawn(async move |this, cx| {
      while let Some(track) = track_stream.next().await {
        let _ = this.update(cx, |this, cx| {
          this.track = track;
          cx.notify();
        });
      }
    }).detach();

    Self {
      track: TrackInfo::default(),
      service: service
    }
  }
}

// Fix the gray uncached album art loading
impl Render for MprisWidget {
  fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    let service = self.service.clone();

    pill()
      .child(
        img(self.track.art_url.clone())
          .size_5()
          .rounded_full()
          .overflow_hidden()
          .border_2()
          .border_color(rgb(0xcba6f7))
          .mr_1()
      )
      .child(if self.track.artist.is_empty() && self.track.title.is_empty() {
        String::from("No media playing")
      } else {
        format!("{} - {}", self.track.artist, self.track.title)
      })
      .on_mouse_down(gpui::MouseButton::Left, move |_, _, cx| {
        let service = service.clone();
        cx.spawn(async move |_| {
          service.play_pause().await;
        }).detach();
      })
      .cursor_pointer()
  }
}
