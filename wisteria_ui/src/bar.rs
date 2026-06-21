use gpui::{
    App, Bounds, Context, DisplayId, Div, Entity, Pixels, Size, Window, WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions, div, layer_shell::*, linear_color_stop, linear_gradient, point, prelude::*, px, rgb, rgba
};

use tracing::info;
use crate::widgets::*;

struct Bar {
  launcher: Entity<LauncherWidget>,
  sys_stats: Entity<SysStatsWidget>,
  music: Entity<MusicWidget>,
  workspaces: Entity<WorkspacesWidget>,
  notification: Entity<NotificationWidget>,
  clock: Entity<ClockWidget>,
  menu: Entity<MenuWidget>
}

impl Bar {
  fn new(cx: &mut Context<Self>) -> Self {
    Self {
        launcher: cx.new(|_cx| LauncherWidget),
        sys_stats: cx.new(|_cx| SysStatsWidget),
        clock: cx.new(ClockWidget::new),
        music: cx.new(MusicWidget::new),
        workspaces: cx.new(|_cx| WorkspacesWidget),
        notification: cx.new(|_cx| NotificationWidget),
        menu: cx.new(|_cx| MenuWidget),
    }
  }
}

impl Render for Bar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
      let root = div().flex().flex_col().size_full();
      let bar = div()
        .h(px(40.0))
        .flex()
        .items_center()
        .justify_center()
        .text_sm()
        .font_family("JetBrainsMono Nerd Font")
        .text_color(rgb(0xcdd6f4))
        .bg(rgba(0x1e1e2ef2));

      let shadow = div()
        .h(px(5.0))
        .bg(
          linear_gradient(
            180.0,
            linear_color_stop(rgba(0x0000007F), 0.0),
            linear_color_stop(rgba(0x00000000), 1.0)
          )
        );
        
      root
        .child(
          bar
          .px_1p5()
          .py_1()
          .child(section(Align::Start, [
            self.launcher.clone().into_any_element(),
            self.sys_stats.clone().into_any_element(),
            self.music.clone().into_any_element(),
          ]))
          .child(section(Align::Center, [
            self.workspaces.clone().into_any_element()
          ]))
          .child(section(Align::End, [
              self.notification.clone().into_any_element(),
              self.clock.clone().into_any_element(),
              self.menu.clone().into_any_element()
          ]))
        )
        .child(shadow)
    }
}

pub fn open_bar(display_id: DisplayId, width: Pixels, cx: &mut App) {
  cx.open_window(
    WindowOptions {
      titlebar: None,
      display_id: Some(display_id),
      window_bounds: Some(WindowBounds::Windowed(Bounds {
        origin: point(px(0.0), px(0.0)),
        size: Size::new(width, px(40.0)),
      })),
      app_id: Some("wisteria_bar".to_string()),
      window_background: WindowBackgroundAppearance::Blurred,
      kind: WindowKind::LayerShell(LayerShellOptions {
        namespace: "bar".to_string(),
        layer: Layer::Top,
        anchor: Anchor::LEFT | Anchor::RIGHT | Anchor::TOP,
        margin: None,
        exclusive_zone: Some(px(35.0)),
        keyboard_interactivity: KeyboardInteractivity::None,
        ..Default::default()
      }),
      ..Default::default()
    },
    |_, cx| cx.new(Bar::new),
  )
  .unwrap();
  info!("Opened bar for display: {:?}", display_id);
}

enum Align {
  Start,
  Center,
  End
}

fn section(align: Align, children: impl IntoIterator<Item = impl IntoElement>) -> Div {
  let root = div().flex().size_full().items_center().gap_1p5().flex_1();
  match align {
    Align::Start => root.justify_start().children(children),
    Align::Center => root.justify_center().children(children),
    Align::End => root.justify_end().children(children),
  }
}
