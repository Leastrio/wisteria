use gpui::{
  AnyView, App, Bounds, Context, DisplayId, Pixels, Size, Window, WindowBackgroundAppearance,
  WindowBounds, WindowKind, WindowOptions, div, layer_shell::*, linear_color_stop, linear_gradient,
  point, prelude::*, px, rgb, rgba,
};

use crate::widgets::*;
use tracing::info;

struct Bar {
  start_widgets: Vec<AnyView>,
  center_widgets: Vec<AnyView>,
  end_widgets: Vec<AnyView>,
}

impl Bar {
  fn new(cx: &mut Context<Self>, display_name: String) -> Self {
    Self {
      start_widgets: vec![
        cx.new(|_cx| LauncherWidget).into(),
        cx.new(SysStatsWidget::new).into(),
        cx.new(MprisWidget::new).into(),
      ],
      center_widgets: vec![cx.new(|cx| WorkspacesWidget::new(cx, display_name)).into()],
      end_widgets: vec![
        cx.new(VpnWidget::new).into(),
        cx.new(PeripheralWidget::new).into(),
        cx.new(ClockWidget::new).into(),
        cx.new(|_cx| MenuWidget).into(),
      ],
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

    let shadow = div().h(px(5.0)).bg(linear_gradient(
      180.0,
      linear_color_stop(rgba(0x0000007F), 0.0),
      linear_color_stop(rgba(0x00000000), 1.0),
    ));

    root
      .child(
        bar
          .child(
            div()
              .absolute()
              .left_0()
              .top_0()
              .bottom_0()
              .px_1p5()
              .py_1()
              .flex()
              .items_center()
              .gap_1p5()
              .px_1p5()
              .children(self.start_widgets.clone()),
          )
          .child(
            div()
              .absolute()
              .left_0()
              .right_0()
              .top_0()
              .bottom_0()
              .px_1p5()
              .py_1()
              .flex()
              .justify_center()
              .items_center()
              .gap_1p5()
              .children(self.center_widgets.clone()),
          )
          .child(
            div()
              .absolute()
              .right_0()
              .top_0()
              .bottom_0()
              .px_1p5()
              .py_1()
              .flex()
              .items_center()
              .gap_1p5()
              .px_1p5()
              .children(self.end_widgets.clone()),
          ),
      )
      .child(shadow)
  }
}

pub fn open_bar(display_id: DisplayId, width: Pixels, display_name: Option<String>, cx: &mut App) {
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
    |_, cx| cx.new(|cx| Bar::new(cx, display_name.unwrap())),
  )
  .unwrap();
  info!("Opened bar for display: {:?}", display_id);
}
