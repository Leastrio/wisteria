use std::sync::Arc;

use futures_signals::signal::SignalExt;
use futures_util::StreamExt;
use gpui::{Context, IntoElement, Render, Window};
use wisteria_services::{
  ServiceRegistry,
  sys_info::{SysInfoService, SysStats},
};

use crate::widgets::*;

pub struct PeripheralWidget {
  //service: Arc<SysInfoService>
}

impl PeripheralWidget {
  pub fn new(cx: &mut Context<Self>) -> Self {
    /*let service = cx.update_global::<ServiceRegistry, _>(|registry, cx| {
      registry.service::<SysInfoService>(cx)
    });

    let mut stats_stream = service.subscribe().to_stream();

    cx.spawn(async move |this, cx| {
      while let Some(stats) = stats_stream.next().await {
        let _ = this.update(cx, |this, cx| {
          cx.notify();
        });
      }
    }).detach();

    Self {
      service
    }*/
    Self {}
  }
}

// Mouse will be green when charging
impl Render for PeripheralWidget {
  fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    pill()
      .child(
        svg()
          .path("icons/mouse.svg")
          .text_color(rgb(0xcdd6f4))
          .size_4()
          .mr_1(),
      )
      .child("57% ")
      .child(
        svg()
          .path("icons/keyboard.svg")
          .text_color(rgb(0xcdd6f4))
          .size_4()
          .mr_1(),
      )
      .child("CLMK")
  }
}
