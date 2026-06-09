use std::{path::Path, time::Duration};

use gpui::{
    App, Bounds, Context, DisplayId, Window, WindowBounds, WindowKind, WindowOptions, div, img, layer_shell::*, prelude::*, px
};

use tracing::info;

struct Wallpaper;

impl Wallpaper {
    fn new(cx: &mut Context<Self>) -> Self {
      cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(100))
                .await;

            let _ = this.update(cx, |_, cx| {
                cx.notify();
            });
        })
        .detach();

      Wallpaper
    }
}

impl Render for Wallpaper {
  fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    div()
      .size_full()
      .overflow_hidden()
      .child(
        img(Path::new("/home/jacob/Wallpapers/tropic_island_night.jpg"))
          .w_full()
          .h_full()
          .object_fit(gpui::ObjectFit::Cover)
      )
  }
}

pub fn open_wallpaper(display_id: DisplayId, cx: &mut App) {
  cx.open_window(
      WindowOptions {
          titlebar: None,
          display_id: Some(display_id),
          window_bounds: Some(WindowBounds::Maximized(Bounds::maximized(Some(display_id), cx))),
          app_id: Some("wisteria_wallpaper".to_string()),
          kind: WindowKind::LayerShell(LayerShellOptions {
              namespace: "wallpaper".to_string(),
              layer: Layer::Background,
              anchor: Anchor::LEFT | Anchor::RIGHT | Anchor::TOP | Anchor::BOTTOM,
              margin: None,
              exclusive_zone: Some(px(-1.0)),
              keyboard_interactivity: KeyboardInteractivity::None,
              ..Default::default()
          }),
          show: true,
          focus: false,
          is_movable: false,
          is_resizable: false,
          is_minimizable: false,
          ..Default::default()
      },
      |_, cx| cx.new(Wallpaper::new),
  )
  .unwrap();

  info!("Opened wallpaper on display: {:?}", display_id);
}
