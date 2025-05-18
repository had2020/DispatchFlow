use rocket::serde::{json::Json, Deserialize};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    launch, options, post, routes, Request, Response,
};

pub mod sql_actions;

use sql_actions::{already_user, clear_user, create_team, create_user, is_user, join_team};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct AccountAction {
    user: String,
    pass: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct TeamsAction {
    user: String,
    pass: String,
    action: String,
    code: String,
    team_name: String,
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

// Create Team
#[post("/create_team", format = "json", data = "<message>")]
fn post_create_team(message: Json<TeamsAction>) -> String {
    create_team(message.user.clone(), message.team_name.clone())
}

#[options("/create_team")]
fn options_create_team() -> &'static str {
    ""
}

// Join Team
#[post("/join_team", format = "json", data = "<message>")]
fn post_join_team(message: Json<TeamsAction>) -> String {
    join_team(
        message.user.clone(),
        message.pass.clone(),
        message.code.clone(),
    )
}

#[options("/join_team")]
fn options_join_team() -> &'static str {
    ""
}

/*
struct Position {
    latitude: f32,
    longitude: f32,
}

struct TeamDuty {
    team_name: String,
    founder_username: String,
    users: Vec<UserDuty>,
}

struct UserDuty {
    username: String,
    online: bool,
    position: Position,
}
*/

// Resister for call
#[post("/get_on_call", format = "json", data = "<message>")]
fn get_on_call(message: Json<TeamsAction>) -> String {
    "no".to_string() //TODO do it in SQL
}

#[options("/get_on_call")]
fn options_get_on_call() -> &'static str {
    ""
}

// CORS
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
    /*
    let mut global_team_status: Vec<TeamDuty> = vec![TeamDuty {
        team_name: "".to_string(),
        founder_username: "".to_string(),
        users: vec![UserDuty {
            username: "".to_string(),
            online: false,
            position: Position {
                latitude: 0.0,
                longitude: 0.0,
            },
        }],
    }];
    */

    rocket::build().attach(CORS).mount(
        "/",
        routes![
            post_signup,
            options_send_signup,
            post_login,
            options_send_login,
            post_clear,
            options_send_clear,
            post_create_team,
            options_create_team,
            post_join_team,
            options_join_team,
            get_on_call,
            options_get_on_call
        ],
    )
}
