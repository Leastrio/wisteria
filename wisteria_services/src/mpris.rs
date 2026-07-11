use std::{
  collections::HashMap,
  sync::{Arc, OnceLock},
};

use futures_signals::signal::{Mutable, MutableSignalCloned};
use futures_util::StreamExt;
use gpui::App;
use tracing::{error, warn};
use zbus::{Connection, Proxy};
use zvariant::OwnedValue;

use crate::{Service, ServiceRegistry};

const SPOTIFY: &str = "org.mpris.MediaPlayer2.spotify";

#[derive(Clone, Default, Debug)]
pub struct TrackInfo {
  pub title: String,
  pub artist: String,
  pub art_url: String,
}

pub struct MprisService {
  conn: OnceLock<Connection>,
  track: Mutable<TrackInfo>,
}

impl Service for MprisService {
  fn new() -> Arc<Self> {
    Arc::new(Self {
      track: Mutable::new(TrackInfo::default()),
      conn: OnceLock::new(),
    })
  }

  fn start(self: Arc<Self>, cx: &mut App) {
    let this = self.clone();

    cx.spawn(async move |cx| {
      let conn = cx.update_global::<ServiceRegistry, _>(|registry, _| registry.session());

      let _ = this.conn.set(conn);

      self.run().await;
    })
    .detach();
  }
}

impl MprisService {
  pub fn subscribe(&self) -> MutableSignalCloned<TrackInfo> {
    self.track.signal_cloned()
  }

  pub async fn play_pause(&self) {
    let _ = Proxy::new(
      self.conn.get().unwrap(),
      "org.mpris.MediaPlayer2.spotify",
      "/org/mpris/MediaPlayer2",
      "org.mpris.MediaPlayer2.Player",
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
      self.conn.get().unwrap(),
      SPOTIFY,
      "/org/mpris/MediaPlayer2",
      "org.freedesktop.DBus.Properties",
    )
    .await?;

    let dbus = Proxy::new(
      self.conn.get().unwrap(),
      "org.freedesktop.DBus",
      "/org/freedesktop/DBus",
      "org.freedesktop.DBus",
    )
    .await?;

    self.track.set(self.get_initial_value().await);

    let mut changes = proxy.receive_signal("PropertiesChanged").await?;
    let mut owners = dbus.receive_signal("NameOwnerChanged").await?;

    loop {
      futures_util::select! {
        msg = changes.next() => {
          let Some(signal) = msg else {
            self.track.set(TrackInfo::default());
            return Ok(());
          };

          let (_iface, changed, _invalided): (String, HashMap<String, OwnedValue>, Vec<String>) = signal.body().deserialize()?;

          if let Some(values) = changed.get("Metadata") {
            let map: HashMap<String, OwnedValue> = values.clone().try_into()?;
            self.track.set(Self::parse_resp(map));
          }
        }

        msg = owners.next() => {
          let Some(signal) = msg else {
            continue;
          };

          let (name, _old, new): (String, String, String) = signal.body().deserialize()?;
          if name == SPOTIFY && new.is_empty() {
            self.track.set(TrackInfo::default());
            return Ok(());
          }
        }
      }
    }
  }

  async fn wait_for_player(&self) -> zbus::Result<()> {
    let dbus = Proxy::new(
      self.conn.get().unwrap(),
      "org.freedesktop.DBus",
      "/org/freedesktop/DBus",
      "org.freedesktop.DBus",
    )
    .await?;

    if dbus
      .call_method("NameHasOwner", &(SPOTIFY))
      .await?
      .body()
      .deserialize::<bool>()?
    {
      return Ok(());
    }

    self.track.set(TrackInfo::default());

    let mut stream = dbus.receive_signal("NameOwnerChanged").await?;
    while let Some(msg) = stream.next().await {
      let (name, _old, new): (String, String, String) = msg.body().deserialize()?;

      if name == SPOTIFY {
        if new.is_empty() {
          self.track.set(TrackInfo::default());
        } else {
          return Ok(());
        }
      }
    }

    unreachable!()
  }

  async fn get_initial_value(&self) -> TrackInfo {
    let proxy = Proxy::new(
      self.conn.get().unwrap(),
      "org.mpris.MediaPlayer2.spotify",
      "/org/mpris/MediaPlayer2",
      "org.mpris.MediaPlayer2.Player",
    )
    .await
    .unwrap();

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
        .unwrap_or_default(),
    }
  }
}
