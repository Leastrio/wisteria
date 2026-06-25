pub mod clock;
pub mod launcher;
pub mod menu;
pub mod mpris;
pub mod notification;
pub mod system_stats;
pub mod workspaces;

use gpui::{Div, FontWeight, Styled, div, prelude::*, rgb, svg};
pub use launcher::LauncherWidget;
pub use system_stats::SysStatsWidget;
pub use mpris::MprisWidget;
pub use workspaces::WorkspacesWidget;
pub use notification::NotificationWidget;
pub use clock::ClockWidget;
pub use menu::MenuWidget;

pub fn pill() -> Div {
  div()
    .px_2()
    .py_0p5()
    .h_full()
    .rounded_full()
    .flex()
    .flex_row()
    .items_center()
    .text_center()
    .bg(rgb(0x313244))
}

pub fn icon_button(path: &'static str) -> Div {
  div()
    .rounded_full()
    .h_full()
    .aspect_square()
    .flex()
    .items_center()
    .justify_center()
    .bg(rgb(0x313244))
    .child(
      svg()
        .path(path)
        .size_4()
        .text_color(rgb(0xcdd6f4))
    )
}
