use rocket::tokio::time::{sleep, Duration};

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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, delay])
}

// // Alternative approach to launch rocket
// #[rocket::main]
// async fn main() -> Result<(), rocket::Error> {
//     let _rocket = rocket::build().mount("/", routes![index]).launch().await?;
//     Ok(())
// }
