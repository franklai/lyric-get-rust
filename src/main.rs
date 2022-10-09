use clap::Parser;
use serde_json::{Result, Value};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    artist: String,
    /// The path to the file to read
    title: String,
}

#[derive(Debug)]
struct SearchResult {
    artist: String,
    title: String,
    id: String,
}

// https://music.line.me/api2/track/mt000000000014f5c8/lyrics.v1
fn get(id: &String) -> String {
    let url = format!("https://music.line.me/api2/track/{id}/lyrics.v1");

    let resp = attohttpc::get(url).send().expect("no more");

    let raw = resp.text_utf8().unwrap();

    let j: Value = serde_json::from_str(&raw).unwrap();
    let lyric = &j["response"]["result"]["lyric"]["lyric"].as_str().unwrap();

    return lyric.to_string();
}

// curl  "https://music.line.me/api2/search/tracks.v1?query=taylor+swift+fifteen&display=3"
// https://music.line.me/webapp/track/mt000000000014f5c8
fn search(artist: String, title: String) -> Vec<SearchResult> {
    let url = "https://music.line.me/api2/search/tracks.v1";
    // let url: String = format!("{url}");
    let resp = attohttpc::get(url)
        .param("query", format!("{artist} {title}"))
        .param("display", "3")
        .send()
        .expect("no more");

    let mut v: Vec<SearchResult> = Vec::new();
    if resp.is_success() {
        let j = resp.text_utf8().expect("what");

        let parsed: Value = serde_json::from_str(&j).unwrap();

        let tracks = &parsed["response"]["result"]["tracks"];

        for track in tracks.as_array().unwrap().iter() {
            let title = &track["trackTitle"].as_str().unwrap();
            let artist = &track["artists"][0]["artistName"].as_str().unwrap();
            let id = &track["trackId"].as_str().unwrap();

            v.push(SearchResult {
                artist: artist.to_string(),
                title: title.to_string(),
                id: id.to_string(),
            });
        }
    }

    return v;
}

fn main() {
    let args = Cli::parse();

    println!("artist {}, title: {}", args.artist, args.title);

    let songs = search(args.artist, args.title);

    // let first_song = &songs[0];
    let lyric = get(&(songs[0].id));
    println!("{lyric}");
}
