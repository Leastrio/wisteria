use std::{collections::HashMap, sync::{Arc, OnceLock}};

use futures_signals::signal::{Mutable, MutableSignalCloned};
use futures_util::StreamExt;
use zbus::{Connection, Proxy};
use zvariant::OwnedValue;

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

impl MprisService {
  pub fn new() -> Arc<Self> {
    Arc::new(Self {
      track: Mutable::new(TrackInfo::default()),
      conn: OnceLock::new()
    })
  }

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
    Proxy::new(
      self.conn().await,
      "org.mpris.MediaPlayer2.spotify",
      "/org/mpris/MediaPlayer2",
      "org.mpris.MediaPlayer2.Player"
    )
    .await
    .unwrap()
    .call_method("PlayPause", &())
    .await
    .unwrap();
  }

  pub async fn run(self: Arc<Self>) {
    let proxy = Proxy::new(
      self.conn().await,
      "org.mpris.MediaPlayer2.spotify",
      "/org/mpris/MediaPlayer2",
      "org.freedesktop.DBus.Properties"
    ).await.unwrap();

    let init = self.get_initial_value().await;
    self.track.set(init);

    let mut changes = proxy.receive_signal("PropertiesChanged").await.unwrap();

    while let Some(signal) = changes.next().await {
      let (_iface, changed, _invalidated): (String, HashMap<String, OwnedValue>, Vec<String>) = signal.body().deserialize().unwrap();

      if let Some(values) = changed.get("Metadata") {
        let v_map: HashMap<String, OwnedValue> = values.clone().try_into().unwrap();
        let info = Self::parse_resp(v_map);
        self.track.set(info);
      }
    }
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
