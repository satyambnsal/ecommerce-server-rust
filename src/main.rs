use rocket::fs::NamedFile;
use rocket::tokio::task::spawn_blocking;
use rocket::tokio::time::{sleep, Duration};
use std::io;
use std::path::{Path, PathBuf};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "E-commerce server is up and running 🚀"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    let vec = spawn_blocking(|| std::fs::read("data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;
    Ok(vec)
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/files/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[get("/user/<id>")]
fn user(id: usize) -> String {
    format!("Executed user handler with id {}", id)
}

#[get("/user/<id>", rank = 2)]
fn user_int(id: isize) -> String {
    format!("Executed user_int handler with id {}", id)
}

#[get("/user/<id>", rank = 3)]
fn user_str(id: &str) -> String {
    format!("Executed user_str handler with id {}", id)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            index,
            delay,
            blocking_task,
            hello,
            files,
            user,
            user_int,
            user_str
        ],
    )
}

// // Alternative approach to launch rocket
// #[rocket::main]
// async fn main() -> Result<(), rocket::Error> {
//     let _rocket = rocket::build().mount("/", routes![index]).launch().await?;
//     Ok(())
// }
