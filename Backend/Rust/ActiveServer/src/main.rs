use rocket::serde::{json::Json, Deserialize};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    launch, options, post, routes, Request, Response,
};

pub mod sql_actions;

use sql_actions::{already_user, clear_user, create_user, is_user};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct AccountAction {
    user: String,
    pass: String,
}

// Signup Route
#[post("/send_signup", format = "json", data = "<message>")]
fn post_signup(message: Json<AccountAction>) -> String {
    let response = format!("{}, {}", message.user, message.pass);
    println!("{}", response);

    if !already_user(message.user.clone()) {
        create_user(message.user.clone(), message.pass.clone());
        "created_user".to_string()
    } else {
        "already_user".to_string()
    }
}

#[options("/send_signup")]
fn options_send_signup() -> &'static str {
    ""
}

// Login Route
#[post("/send_login", format = "json", data = "<message>")]
fn post_login(message: Json<AccountAction>) -> String {
    let response = format!("{}, {}", message.user, message.pass);

    if is_user(message.user.clone(), message.pass.clone()) {
        "is_user".to_string()
    } else {
        "not_user".to_string()
    }
}

#[options("/send_login")]
fn options_send_login() -> &'static str {
    ""
}

// Clear User Route
#[post("/send_clear", format = "json", data = "<message>")]
fn post_clear(message: Json<AccountAction>) -> String {
    let response = format!("{}, {}", message.user, message.pass);

    if clear_user(message.user.clone()) {
        "cleared".to_string()
    } else {
        "failed".to_string()
    }
}

#[options("/send_clear")]
fn options_send_clear() -> &'static str {
    ""
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, OPTIONS",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        ));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(CORS).mount(
        "/",
        routes![
            post_signup,
            options_send_signup,
            post_login,
            options_send_login,
            post_clear,
            options_send_clear
        ],
    )
}
