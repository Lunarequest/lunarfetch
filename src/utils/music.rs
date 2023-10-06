use std::collections::HashMap;

use anyhow::{Context, Result};
use zbus::{fdo::DBusProxy, names::OwnedBusName, Connection};
use zvariant::Value;

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

    let body = message.body::<Value>()?;

    let seralised_body: HashMap<String, Value> = body.try_into()?;

    let title: String = seralised_body
        .get("xesam:title")
        .context("no title, this shouldn't be possible")?
        .try_into()?;
    Ok(title)
}

pub async fn get_song_dbus() -> Result<String> {
    let connection = Connection::session().await?;
    let proxy = DBusProxy::new(&connection).await?;

    let m = proxy.list_names().await?;
    let filtered_m = m
        .into_iter()
        .filter(|e| {
            e.contains("org.mpris.MediaPlayer2.") && (!e.contains("chrom") || !e.contains("kde"))
        })
        .collect::<Vec<OwnedBusName>>();

    if filtered_m.len() > 1 {
        for bus in filtered_m {
            let music = music_from_bus(bus, connection).await?;
            return Ok(music);
        }
        return Ok("".to_string());
    } else {
        return Ok("".to_string());
    }
}
