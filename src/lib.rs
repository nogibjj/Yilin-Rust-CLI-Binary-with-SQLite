// lib.rs
use rusqlite::{params, Connection, Result, NO_PARAMS};
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Song {
    pub spotify_id: String,
    pub name: String,
    pub artists: String,
    pub daily_rank: String,
    pub daily_movement: String,
    pub weekly_movement: String,
    pub country: String,
    pub snapshot_date: String,
    pub popularity: String,
    pub is_explicit: bool,
    pub duration_ms: i64,
    pub album_name: String,
    pub album_release_date: String,
    pub danceability: f64,
    pub energy: f64,
    pub key: i32,
    pub loudness: f64,
    pub mode: i32,
    pub speechiness: f64,
    pub acousticness: f64,
    pub instrumentalness: f64,
    pub liveness: f64,
    pub valence: f64,
    pub tempo: f64,
    pub time_signature: String,
}
pub fn establish_connection() -> Result<Connection> {
    Connection::open("spotify_tracks.db")
}

pub fn create_track(conn: &Connection, track: &Track) -> Result<()> {
    let query = "INSERT INTO tracks (spotify_id, name, artists) VALUES (?1, ?2, ?3)"; // add all needed fields
    conn.execute(query, params![track.spotify_id, track.name, track.artists])?; // add all needed fields
    Ok(())
}

pub fn read_tracks(conn: &Connection) -> Result<Vec<Track>> {
    let mut stmt = conn.prepare("SELECT spotify_id, name, artists FROM tracks")?; // adjust for your query
    let tracks_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(Track {
            spotify_id: row.get(0)?,
            name: row.get(1)?,
            artists: row.get(2)?,
            // ... other fields
        })
    })?;

    let mut tracks = Vec::new();
    for track in tracks_iter {
        tracks.push(track?);
    }
    Ok(tracks)
}

pub fn update_track(conn: &Connection, track: &Track) -> Result<()> {
    let query = "UPDATE tracks SET name = ?2, artists = ?3 WHERE spotify_id = ?1"; // add all needed fields
    conn.execute(query, params![track.spotify_id, track.name, track.artists])?; // add all needed fields
    Ok(())
}

pub fn delete_track(conn: &Connection, spotify_id: &str) -> Result<()> {
    let query = "DELETE FROM tracks WHERE spotify_id = ?1";
    conn.execute(query, params![spotify_id])?;
    Ok(())
}
