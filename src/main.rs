use axum::{
    extract,
    routing::{get, post},
    Json, Router,
};
use redis::Commands;
use std::{collections::HashSet, net::SocketAddr};

#[tokio::main]
async fn main() {
    // connect to redis

    let app = Router::new()
        .route("/", get(handler))
        .route("/login", post(login))
        .route("/session", get(session));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3005));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, world!"
}

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct CreateUser {
    email: String,
    password: String,
}

async fn login(extract::Json(payload): extract::Json<CreateUser>) -> Json<CreateUser> {
    let client = redis::Client::open("redis://127.0.0.1/").expect("er redis");
    let mut con = client.get_connection().expect("er redis");
    let _: () = con
        .set(payload.email.clone(), payload.password.clone())
        .expect("err");

    let py = con.get::<_, String>(payload.email.clone()).expect("err");
    println!("{} days", py);
    println!("{:?}", payload);
    Json(CreateUser {
        email: payload.email,
        password: payload.password,
    })
}

async fn session() -> &'static str {
    let redis = redis::Client::open("redis://127.0.0.1/").expect("er redis");
    let mut conn = redis.get_connection().expect("er redis");

    let _: () = redis::cmd("SET")
        .arg("foo")
        .arg("bar")
        .query(&mut conn)
        .expect("failed to execute SET for 'foo'");

    let _: () = redis::cmd("HSET")
        .arg("user1")
        .arg("Name")
        .arg("Hossam")
        .arg("Month")
        .arg("April")
        .query(&mut conn)
        .expect("failed to execute HSET");

    let user1: Vec<(String, String)> = redis::cmd("HGETALL")
        .arg("user1")
        .query(&mut conn)
        .expect("failed to execute HGetall");

    println!("{:?}", user1[0].1);

    "session"
}
