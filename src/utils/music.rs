use core::fmt;
use std::collections::HashMap;

use anyhow::{Context, Result};
use zbus::{fdo::DBusProxy, names::OwnedBusName, Connection};
use zvariant::{Array, Value};

#[derive(Debug)]
pub struct Music {
    title: String,
    artist: String,
    album: String,
}

impl fmt::Display for Music {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {} - {}", self.title, self.album, self.artist)
    }
}

async fn music_from_bus(interface: OwnedBusName, connection: Connection) -> Result<String> {
    let message = connection
        .call_method(
            Some(interface),
            "/org/mpris/MediaPlayer2",
            Some("org.freedesktop.DBus.Properties"),
            "Get",
            &("org.mpris.MediaPlayer2.Player", "Metadata"),
        )
        .await?;

    let body = message.body();
    let mut muisc = Music {
        title: "Unknown Song".into(),
        artist: "Unknown Artist".into(),
        album: "Unknown Album".into(),
    };

    let seralised_body: Value = body.deserialize()?;
    let maped_body: HashMap<String, Value> = seralised_body.try_into()?;
    let title: String = maped_body
        .get("xesam:title")
        .context("no title, this shouldn't be possible")?
        .try_into()?;
    muisc.title = title;

    let album = maped_body
        .get("xesam:album")
        .and_then(|v| v.try_into().ok())
        .unwrap_or_else(|| "Unknown album".to_string());
    muisc.album = album;

    let artist = maped_body
        .get("xesam:artist")
        .and_then(|v| TryInto::<Array>::try_into(v).ok())
        .map(|array| {
            array
                .iter()
                .filter_map(|v| TryInto::<String>::try_into(v).ok())
                .collect::<Vec<_>>()
                .join(" & ")
        })
        .unwrap_or_else(|| "Unknown artist".to_string());
    muisc.artist = artist;

    Ok(muisc.to_string())
}

pub async fn get_song_dbus() -> Result<String> {
    let connection = Connection::session().await?;
    let proxy = DBusProxy::new(&connection).await?;

    let m = proxy.list_names().await?;
    let filtered_m = m
        .into_iter()
        .filter(|e| {
            e.contains("org.mpris.MediaPlayer2.")
                && !e.contains("chromium")
                && !e.contains("kdeconnect")
        })
        .collect::<Vec<OwnedBusName>>();

    if !filtered_m.is_empty() {
        for bus in filtered_m {
            let music = music_from_bus(bus, connection.clone()).await?;
            if music.len() > 2 {
                // simple check to iterate through all buses
                return Ok(music);
            }
        }
        Ok("Unknown Artist - Unknown Album - Unknown Song".to_string())
    } else {
        Ok("Unknown Artist - Unknown Album - Unknown Song".to_string())
    }
}
