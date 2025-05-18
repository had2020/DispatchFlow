use rocket::response::status;
use rocket::serde::{json::Json, Deserialize};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    launch, options, post, routes, Request, Response,
};

use std::fs::OpenOptions;
use std::io::Write;

pub mod sql_actions;

use sql_actions::{
    add_chat_message, add_position, already_user, check_all_team_status, check_duty_status,
    check_team_chat, check_team_positions, clear_user, create_team, create_user, is_user,
    join_team, set_duty_status,
};

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

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct DutyAction {
    user: String,
    team_name: String,
    status: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ChatAction {
    user: String,
    team_name: String,
    message: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct PositionAction {
    user: String,
    team_name: String,
    latitude: String,
    longitude: String,
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
#[post("/send_on_call", format = "json", data = "<message>")]
fn send_on_call(message: Json<DutyAction>) -> String {
    /*
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("/on_duty_users.txt")
        .unwrap();
    writeln!(file, "{}", message.user).expect("Failed to write file");
    */

    set_duty_status(
        message.user.clone(),
        message.team_name.clone(),
        message.status.clone(),
    );

    "good".to_string()
}

#[options("/send_on_call")]
fn options_send_on_call() -> &'static str {
    ""
}

// Check for call
#[post("/check_on_call", format = "json", data = "<message>")]
fn check_on_call(message: Json<DutyAction>) -> String {
    check_duty_status(message.user.clone())
}

#[options("/check_on_call")]
fn options_check_on_call() -> &'static str {
    ""
}

// Check all team members status
#[post("/check_all_status", format = "json", data = "<message>")]
fn check_all_status(message: Json<DutyAction>) -> String {
    check_all_team_status(message.team_name.clone())
}

#[options("/check_all_status")]
fn options_check_all_status() -> &'static str {
    ""
}

// Send Chat Message
#[post("/send_chat_message", format = "json", data = "<message>")]
fn send_chat_message(message: Json<ChatAction>) -> String {
    add_chat_message(
        message.user.clone(),
        message.team_name.clone(),
        message.message.clone(),
    );
    "sent".to_string()
}

#[options("/send_chat_message")]
fn options_send_chat_message() -> &'static str {
    ""
}

// Check Chat
#[post("/check_chat", format = "json", data = "<message>")]
fn check_chat(message: Json<ChatAction>) -> String {
    check_team_chat(message.team_name.clone())
}

#[options("/check_chat")]
fn options_check_team_chat() -> &'static str {
    ""
}

// Send Position
#[post("/send_position", format = "json", data = "<message>")]
fn send_position(message: Json<PositionAction>) -> String {
    add_position(
        message.user.clone(),
        message.team_name.clone(),
        message.latitude.clone(),
        message.longitude.clone(),
    );
    "Sent Position".to_string()
}

#[options("/send_position")]
fn options_send_position() -> &'static str {
    ""
}

// Check Positions
#[post("/check_positions", format = "json", data = "<message>")]
fn check_positions(message: Json<PositionAction>) -> String {
    check_team_positions(message.team_name.clone())
}

#[options("/check_positions")]
fn options_check_positions() -> &'static str {
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
            send_on_call,
            options_send_on_call,
            check_on_call,
            options_check_on_call,
            check_all_status,
            options_check_all_status,
            send_chat_message,
            options_send_chat_message,
            check_chat,
            options_check_team_chat,
            send_position,
            options_send_position,
            check_positions,
            options_check_positions
        ],
    )
}
