use std::{path::PathBuf, sync::Arc, time::Duration};

use gpui::{
    App, Application
};

use reqwest_client::ReqwestClient;
use tracing::info;
use tracing_subscriber::EnvFilter;
use clap::{Parser, Subcommand};

/// Wisteria Wayland shell
#[derive(Parser, Debug)]
struct Cli {
  #[command(subcommand)]
  command: Option<Command>,

  /// Path to override the default config file
  #[arg(short, long)]
  config: Option<PathBuf>
}

#[derive(Subcommand, Debug)]
enum Command {
  /// Send commands to a running Wisteria instance
  Ipc {
    #[command(subcommand)]
    command: IpcCommand
  }
}

#[derive(Subcommand, Debug)]
enum IpcCommand {
  /// Toggle the launcher window
  ToggleLauncher
}

fn main() {
  #[cfg(not(all(target_os = "linux", unix)))]
  compile_error!("This application only supports Linux.");

  if std::env::var("WAYLAND_DISPLAY").is_err() {
    eprintln!("Wisteria requires a Wayland session.");
    std::process::exit(1);
  }

  tracing_subscriber::fmt().with_env_filter(EnvFilter::new("wisteria=trace")).init();

  let cli = Cli::parse();
  match cli.command {
    None => start_shell(cli.config),
    Some(Command::Ipc { command }) => match command {
      IpcCommand::ToggleLauncher => todo!()
    }
  }
}

fn start_shell(config: Option<PathBuf>) {
  Application::with_platform(gpui_linux::current_platform(false))
    .with_assets(wisteria_ui::assets::Assets)
    .run(|cx: &mut App| {
      let http_client = ReqwestClient::user_agent("wisteria").unwrap();
      cx.set_http_client(Arc::new(http_client));

      cx.spawn(async move |cx| {
        cx.background_executor().timer(Duration::from_millis(100)).await;
        cx.update(|cx: &mut App| {
          let displays = cx.displays();

          for d in displays {
            wisteria_ui::bar::open_bar(d.id(), d.bounds().size.width, cx);
            wisteria_ui::wallpaper::open_wallpaper(d.id(), cx);
          }
        });
      }).detach(); 

      info!("Wisteria launched");
  });
}
