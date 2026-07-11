use std::sync::Arc;

use futures_signals::signal::SignalExt;
use futures_util::StreamExt;
use gpui::{Context, IntoElement, Render, Window};
use wisteria_services::{
  ServiceRegistry,
  sys_info::{SysInfoService, SysStats},
};

use crate::widgets::*;

pub struct VpnWidget {
  //service: Arc<SysInfoService>
}

// Possibly use dbus network manager for state and then spawn protonvpn cli
impl VpnWidget {
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

// vpn.svg + city when connected vpn_off.svg when disconnected
impl Render for VpnWidget {
  fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    pill().child(
      svg()
        .path("icons/vpn_off.svg")
        .text_color(rgb(0xcdd6f4))
        .size_4(),
    )
  }
}
