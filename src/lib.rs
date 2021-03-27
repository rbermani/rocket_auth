//! rocket_auth provides a backend agnostic API for authentication management.
//! It supports connections for SQLite and Postgresql. It lets you create, delete, and authenticate users.
//! The available features are:
//! * `sqlite-db`: for interacting with a SQLite database. 
//! * `postgres-db`: for interacting with a Postgresql database.
//! * `redis-session`: for storing sessions on a redis server. 
//! 
//! By default this crate stores sessions on a concurrent hashmap. 
//! As a result, sessions will only be stored as long as the rocket application runs uninterrupted. 
//! In order to store sessions more robustly, it is recommended to connect the [`Users`] instance to a redis server.
//! This requires the `redis-session` feature to be enabled. 
//! 
//! `rocket_auth` uses private cookies to store session data. 
//! This means that in order for cookies to be properly decrypted between launches, a `secret_key` must be set.
//! For more information visit rocket's [configuration guide](https://rocket.rs/v0.4/guide/configuration/).
//! 
//! 
//! 
//! 
//! 
//! To use `rocket_auth` include it as a dependency in your Cargo.toml file: 
//! ```ini
//! [dependencies.rocket_auth]
//! version = "0.1"
//! features = ["sqlite-db"]
//! ```
//! # Quick overview
//! This crate provides two guards:
//! * [`Auth`]: manages authentication.
//! * [`Session`]: retrieves session data from client cookies.
//! 
//! It also includes two structs to be parsed from forms and json data: 
//! * [`Signup`]: used to create new users.
//! * [`Login`]: used to authenticate users.
//! 
//! Finally it has two structures for queries: 
//! * [`Users`]: it allows to query users to the database.
//! * [`User`]: it is the response of a query.
//! 
//! ## Auth guard
//! The [`Auth`] guard allows to log in, log out, sign up, modify, and delete the currently (un)authenticated user. 
//! For more information see [`Auth`]. A working example: 
//! ```rust,no_run
//! use rocket::{get, post, Form};
//! use rocket_auth::{Users, Error, Auth, Signup, Login};
//! 
//! #[post("/signup", data="<form>")] 
//! fn signup(form: Form<Signup>, auth: Auth) {
//!     // users are automatically logged in after signing up.
//!     auth.signup(&form);
//! }
//! 
//! #[post("/login", data="<form>")] 
//! fn login(form: Form<Login>, auth: Auth) {
//!     auth.login(&form);
//! }
//! 
//! #[get("/logout")] 
//! fn logout(auth: Auth) {
//!     auth.logout();
//! }
//! //! #[get("/delete_account")] 
//! fn delete(auth: Auth) {
//!     auth.delete();
//! }
//! 
//! #[get("/am-I-authenticated")] 
//! fn is_auth(auth: Auth) -> &'static str {
//!     if auth.is_auth() {
//!         "Yes you are."
//!     } else {
//!         "nope."
//!     }
//! }
//! 
//! fn main() -> Result<(), Error>{
//!     let users = Users::open_sqlite("mydb.db")?;
//! 
//!     rocket::ignite()
//!         .mount("/", routes![signup, login, logout, delete, is_auth])
//!         .manage(users)
//!         .launch();
//!     Ok(())
//! }
//! ```
//! # Users struct
//! The [`Users`] struct administers interactions with the database. 
//! It lets you query, create, modify and delete users.
//! Unlike the [`Auth`] guard, a [`Users`] is instance can manage any user in the database.
//! A simple example of how to query a user with the [`Users`] struct:
//! 
//! ```rust 
//! #[get("see-user/<id>")]
//! fn see_user(id: u32, users: State<Users>) -> String {
//!     let user = users.get_by_id(id);
//!     fortmat!("{}", json!(user))
//! }
//! ```
//! 
//! A [`Users`] instance can be constructed by connecting it to the database with the methods [`open_sqlite`](Users::open_sqlite),
//! [`open_postgres`](Users::open_postgres). Furthermore, it can be constructed from a working connection. 




mod cookies;
mod db;
mod error;
mod forms;
mod prelude;
mod session;
mod user;

#[cfg(test)]
mod tests;


use prelude::*;
use rocket::FromForm;


pub use cookies::Session;
pub use error::Error;
pub use crate::user::auth::Auth;


pub type Result<T, E = Error> = std::result::Result<T, E>;





#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct User {
    pub id: u32,
    pub email: String,
    #[serde(skip_serializing)]
    password: String,
    pub is_admin: bool,
}



pub struct Users {
    conn: Box<dyn DBConnection>,
    sess: Box<dyn SessionManager>,
}


#[derive(FromForm, Deserialize, Debug)]
pub struct Login {
    pub email: String,
    password: String,
}

#[derive(FromForm, Deserialize, Debug)]
pub struct Signup {
    pub email: String,
    password: String,
}
