use actix_web::get;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::Responder;
use image::imageops;
use image::Rgba;
use image::RgbaImage;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use std::fs::read_dir;
use std::fs::remove_file;
use std::io;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use uuid::Uuid;

lazy_static! {
    static ref MUTEX: Mutex<i32> = Mutex::new(0);
}

static mut DELETION_QUEUE: Vec<String> = vec![];
static mut EMOJIS: Vec<(String, String)> = vec![];
static POSITIONS: [(i64, i64); 6] = [
    (50, 50),
    (260, 50),
    (470, 50),
    (50, 260),
    (266, 260),
    (470, 260),
];

#[actix_web::main]
async fn main() -> io::Result<()> {
    let files = read_dir("emoji-data-ios/img-apple-160")?
        .map(|res| res.map(|e| e.path().into_os_string().into_string().unwrap()))
        .collect::<io::Result<Vec<_>>>()?;
    let codes = files
        .iter()
        .map(|e| {
            e.split("/")
                .last()
                .unwrap()
                .split_once(".")
                .unwrap()
                .0
                .to_string()
        })
        .collect::<Vec<_>>();
    for (i, c) in codes.iter().enumerate() {
        if !c.contains("1f1") {
            unsafe {
                EMOJIS.push((c.to_owned(), files[i].to_owned()));
            }
        }
    }
    thread::spawn(move || loop {
        let _ = MUTEX.lock();
        thread::sleep(Duration::from_secs(5));
        unsafe {
            while let Some(path) = DELETION_QUEUE.pop() {
                remove_file(path).unwrap_or_default();
            }
        }
    });
    HttpServer::new(|| App::new().service(handle_request))
        .bind(("127.0.0.1", 8080))? // TODO: the addr might be configurable
        .run()
        .await
}

#[get("/")]
async fn handle_request() -> impl Responder {
    let _ = MUTEX.lock();
    let mut emojis = unsafe {
        EMOJIS
            .choose_multiple(&mut rand::thread_rng(), 9)
            .cloned()
            .collect::<Vec<_>>()
    };
    let correct_emojis = &emojis.clone()[0..6];
    let mut image = RgbaImage::from_fn(680, 470, |_, _| Rgba([0, 0, 0, 255]));
    imageops::vertical_gradient(
        &mut image,
        &Rgba([20, 20, 20, 255]),
        &Rgba([25, 25, 25, 255]),
    );
    for i in 0..6 {
        let path = &emojis.get(i).unwrap().1;
        let mut emoji = image::open(path).unwrap().into_rgba8();
        let (x, y) = POSITIONS[i];
        imageops::overlay(&mut image, &mut emoji, x, y)
    }
    let path = format!("{}.png", Uuid::new_v4().to_string());
    unsafe {
        DELETION_QUEUE.push(path.clone());
    }
    image.save(&path).unwrap();
    emojis.shuffle(&mut rand::thread_rng());
    actix_files::NamedFile::open(path.as_str())
        .customize()
        .insert_header((
            "x-emojis",
            emojis
                .iter()
                .map(|e| e.0.as_str())
                .collect::<Vec<_>>()
                .join(";")
                .as_str(),
        ))
        .insert_header((
            "x-correct-emojis",
            correct_emojis
                .iter()
                .map(|e| e.0.as_str())
                .collect::<Vec<_>>()
                .join(";")
                .as_str(),
        ))
}
