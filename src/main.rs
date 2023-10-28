// main.rs
mod lib; // this refers to your lib.rs
use crate::lib::{create_track, delete_track, establish_connection, read_tracks, update_track, Track};
use rusqlite::Result;

fn main() -> Result<()> {
    let conn = establish_connection()?;

    // Create a new track
    let new_track = Track {
        spotify_id: "some_spotify_id".to_string(),
        name: "some track name".to_string(),
        artists: "some artist".to_string(),
        // ... other fields
    };
    create_track(&conn, &new_track)?;

    // Read all tracks
    let tracks = read_tracks(&conn)?;
    for track in &tracks {
        println!("{:?}", track);
    }

    // Update a track
    let updated_track = Track {
        spotify_id: "some_spotify_id".to_string(),
        name: "new track name".to_string(),
        artists: "new artist".to_string(),
        // ... other fields
    };
    update_track(&conn, &updated_track)?;

    // Delete a track
    delete_track(&conn, "some_spotify_id")?;

    Ok(())
}
