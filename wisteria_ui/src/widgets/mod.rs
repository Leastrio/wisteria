pub mod clock;
pub mod launcher;
pub mod menu;
pub mod music;
pub mod notification;
pub mod system_stats;
pub mod workspaces;
pub mod volume;

use gpui::{Div, FontWeight, Styled, div, prelude::*, rgb, svg};
pub use launcher::LauncherWidget;
pub use system_stats::SysStatsWidget;
pub use music::MusicWidget;
pub use workspaces::WorkspacesWidget;
pub use notification::NotificationWidget;
pub use volume::VolumeWidget;
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

pub fn workspace_button(active: bool, num: usize) -> Div {
    div()
        .size_6()
        .rounded_full()
        .flex()
        .items_center()
        .justify_center()
        .font_weight(FontWeight::SEMIBOLD)
        .bg(if active {
            rgb(0xcba6f7)
        } else {
            rgb(0x45475a)
        })
        .text_color(if active {
            rgb(0x1e1e2e)
        } else {
            rgb(0xcdd6f4)
        })
        .child(num.to_string())
}
