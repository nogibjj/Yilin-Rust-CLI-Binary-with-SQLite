// lib.rs
use rusqlite::{params, Connection, Result};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

#[derive(Debug, Deserialize, Clone)]
pub struct Track {
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

pub fn extract(file_path: &str) -> Result<Vec<Track>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let mut tracks = Vec::new();

    for result in rdr.deserialize() {
        let record: Track = result?;
        tracks.push(record);
    }

    Ok(tracks)
}

pub fn transform(tracks: &[Track]) -> Vec<Track> {
    // Here, you can transform your data as needed, this function just clones data as-is
    tracks.to_vec()
}

// Change the function signature to accept a mutable reference
pub fn load(conn: &mut Connection, tracks: &[Track]) -> Result<(), rusqlite::Error> {
    let tx = conn.transaction()?;
    
    for track in tracks {
        // Construct your INSERT query and parameters based on your Track structure
        tx.execute(
            "INSERT INTO tracks (spotify_id, name, artists) VALUES (?1, ?2, ?3)", 
            params![track.spotify_id, track.name, track.artists],
        )?;
    }
    
    tx.commit()
}

