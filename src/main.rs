#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "E-commerce server is up and running 🚀"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
