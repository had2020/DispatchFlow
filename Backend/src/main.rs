use rocket::serde::{json::Json, Deserialize};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    launch, options, post, routes, Request, Response,
};

pub mod sql_actions;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct AccountAction {
    action: String,
    user: String,
    pass: String,
}

#[post("/send_signup", format = "json", data = "<message>")]
fn post_signup(message: Json<AccountAction>) -> String {
    let response = format!("{}, {}, {}", message.action, message.user, message.pass);
    println!("{}", response);
}
