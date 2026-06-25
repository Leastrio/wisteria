use std::{collections::HashMap, sync::{Arc, OnceLock}};

use futures_signals::signal::{Mutable, MutableSignalCloned};
use futures_util::StreamExt;
use tracing::{error, warn};
use zbus::{Connection, Proxy};
use zvariant::OwnedValue;
use gpui::App;

use crate::Service;

const SPOTIFY: &str = "org.mpris.MediaPlayer2.spotify";

#[derive(Clone, Default, Debug)]
pub struct TrackInfo {
  pub title: String,
  pub artist: String,
  pub art_url: String
}

pub struct MprisService {
  track: Mutable<TrackInfo>,
  conn: OnceLock<Connection>
}

impl Service for MprisService {
  fn new() -> Arc<Self> {
    Arc::new(Self {
      track: Mutable::new(TrackInfo::default()),
      conn: OnceLock::new()
    })
  }

  fn start(self: Arc<Self>, cx: &mut App) {
    cx.spawn(async move |_| {
      self.run().await;
    }).detach();
  }
}

impl MprisService {
  async fn conn(&self) -> &Connection {
    if let Some(conn) = self.conn.get() {
      return conn;
    }

    let conn = Connection::session().await.unwrap();
    let _ = self.conn.set(conn);
    self.conn.get().unwrap()
  }

  pub fn subscribe(&self) -> MutableSignalCloned<TrackInfo> {
    self.track.signal_cloned()
  }

  pub async fn play_pause(&self) {
    let _ = Proxy::new(
      self.conn().await,
      "org.mpris.MediaPlayer2.spotify",
      "/org/mpris/MediaPlayer2",
      "org.mpris.MediaPlayer2.Player"
    )
    .await
    .unwrap()
    .call_method("PlayPause", &())
    .await;
  }

  pub async fn run(self: Arc<Self>) {
    loop {
      match self.wait_for_player().await {
        Ok(_) => {
          if let Err(err) = self.watch_player().await {
            warn!(?err, "spotify listener stopped");
          }
        }
        Err(err) => {
          error!(?err, "failed waiting for spotify");
        }
      }
    }
  }

  pub async fn watch_player(&self) -> zbus::Result<()> {
    let proxy = Proxy::new(
      self.conn().await,
      SPOTIFY,
      "/org/mpris/MediaPlayer2",
      "org.freedesktop.DBus.Properties"
    ).await?;

    self.track.set(self.get_initial_value().await);

    let mut changes = proxy.receive_signal("PropertiesChanged").await?;

    while let Some(signal) = changes.next().await {
      let (_iface, changed, _invalidated): (String, HashMap<String, OwnedValue>, Vec<String>) = signal.body().deserialize()?;

      if let Some(values) = changed.get("Metadata") {
        let v_map: HashMap<String, OwnedValue> = values.clone().try_into()?;
        let info = Self::parse_resp(v_map);
        self.track.set(info);
      }
    }

    Ok(())
  }

  async fn wait_for_player(&self) -> zbus::Result<()> {
    let dbus = Proxy::new(
      self.conn().await,
      "org.freedesktop.DBus",
      "/org/freedesktop/DBus",
      "org.freedesktop.DBus"
    ).await?;

    if dbus.call_method("NameHasOwner", &(SPOTIFY)).await?.body().deserialize::<bool>()? {
      return Ok(());
    }

    let mut stream = dbus.receive_signal("NameOwnerChanged").await?;
    while let Some(msg) = stream.next().await {
      let (name, _old, new): (String, String, String) = msg.body().deserialize()?;

      if name == SPOTIFY && !new.is_empty() {
        return Ok(());
      }
    }

    unreachable!()
  }

  async fn get_initial_value(&self) -> TrackInfo {
    let proxy = Proxy::new(
      self.conn().await,
      "org.mpris.MediaPlayer2.spotify",
      "/org/mpris/MediaPlayer2",
      "org.mpris.MediaPlayer2.Player"
    ).await.unwrap();

    let resp: HashMap<String, OwnedValue> = proxy.get_property("Metadata").await.unwrap();
    return Self::parse_resp(resp);
  }

  fn parse_resp(resp: HashMap<String, OwnedValue>) -> TrackInfo {
    TrackInfo {
      title: resp
        .get("xesam:title")
        .and_then(|v| String::try_from(v.clone()).ok())
        .unwrap_or_default(),
      artist: resp
        .get("xesam:artist")
        .and_then(|v| Vec::<String>::try_from(v.clone()).ok())
        .map(|artists| artists.join(", "))
        .unwrap_or_default(),
      art_url: resp
        .get("mpris:artUrl")
        .and_then(|v| String::try_from(v.clone()).ok())
        .unwrap_or_default()
    }
  }
}
