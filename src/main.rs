use rocket::{serde::{Serialize,Deserialize, json::Json}, State};
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
struct Music {
    id: usize,
    title: String,
    artist: String,
    album: Option<String>,
    genre: String,
    duration: f32,
}

struct MusicStore {
    musics: Arc<Mutex<Vec<Music>>>, // Wrap the vector in an Arc<Mutex>
}

#[get("/")]
fn list_all(music_store: &State<MusicStore>) -> Json<Vec<Music>> {
    let musics = music_store.musics.lock().unwrap();
    Json(musics.clone())
}

#[get("/<id>")]
fn get_music_by_id(id: usize, music_store: &State<MusicStore>) -> Json<Music> {
    let musics = music_store.musics.lock().unwrap();
    let music = musics.iter().find(|music| music.id == id).unwrap();
    Json(music.clone())
}

#[post("/", data = "<new_music>")]
fn add_music(new_music: Json<Music>, music_store: &State<MusicStore>) -> Json<Music> {
    let mut musics = music_store.musics.lock().unwrap();
    let new_id = musics.iter().map(|music| music.id).max().unwrap_or(0) + 1;
    let mut music = new_music.into_inner();
    music.id = new_id;
    musics.push(music.clone());
    Json(music)
}

#[put("/<id>", data = "<music>")]
fn update_music(id: usize, music: Json<Music>, music_store: &State<MusicStore>) -> Json<Music> {
    let mut musics = music_store.musics.lock().unwrap();
    let index = musics.iter().position(|music| music.id == id).unwrap();
    musics[index] = music.into_inner();
    Json(musics[index].clone())
}

#[delete("/<id>")]
fn delete_music(id: usize, music_store: &State<MusicStore>) -> Json<usize> {
    let mut musics = music_store.musics.lock().unwrap();
    let index = musics.iter().position(|music| music.id == id).unwrap();
    musics.remove(index);
    Json(id)
}

#[rocket::main]
async fn main() {    
    let musics: Vec<Music> = vec![
        Music {
            id: 1,
            title: "The Less I Know The Better".to_string(),
            artist: "Tame Impala".to_string(),
            album: Some("Currents".to_string()),
            genre: "Rock".to_string(),
            duration: 216.2,
        },
        Music {
            id: 2,
            title: "The Less I Know The Better".to_string(),
            artist: "Tame Impala".to_string(),
            album: Some("Currents".to_string()),
            genre: "Rock".to_string(),
            duration: 216.2,
        },
        Music {
            id: 3,
            title: "The Less I Know The Better".to_string(),
            artist: "Tame Impala".to_string(),
            album: Some("Currents".to_string()),
            genre: "Rock".to_string(),
            duration: 216.2,
        },
        Music {
            id: 4,
            title: "The Less I Know The Better".to_string(),
            artist: "Tame Impala".to_string(),
            album: Some("Currents".to_string()),
            genre: "Rock".to_string(),
            duration: 216.2,
        },
        Music {
            id: 5,
            title: "The Less I Know The Better".to_string(),
            artist: "Tame Impala".to_string(),
            album: Some("Currents".to_string()),
            genre: "Rock".to_string(),
            duration: 216.2,
        },
        Music {
            id: 6,
            title: "The Less I Know The Better".to_string(),
            artist: "Tame Impala".to_string(),
            album: Some("Currents".to_string()),
            genre: "Rock".to_string(),
            duration: 216.2,
        },
        Music {
            id: 7,
            title: "The Less I Know The Better".to_string(),
            artist: "Tame Impala".to_string(),
            album: Some("Currents".to_string()),
            genre: "Rock".to_string(),
            duration: 216.2,
        },
        Music {
            id: 8,
            title: "The Less I Know The Better".to_string(),
            artist: "Tame Impala".to_string(),
            album: Some("Currents".to_string()),
            genre: "Rock".to_string(),
            duration: 216.2,
        },
    ];

     let music_store = MusicStore {
        musics: Arc::new(Mutex::new(musics)), // Wrap the vector in an Arc<Mutex>
    };

    rocket::build()
        .manage(music_store)
        .mount("/musicas", routes![list_all, get_music_by_id, add_music, update_music, delete_music])
        .launch()
        .await
        .expect("Rocket failed to launch");
}