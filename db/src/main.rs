use axum::Router;
use axum::routing::get;
use std::sync::LazyLock;
use surrealdb::{
  Surreal,
  engine::remote::ws::{Client, Ws},
};

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // surreal db
  DB.connect::<Ws>("0.0.0.0:8000").await?;
  // use tracing
  println!("db operational");

  // // axum server
  // let listener = tokio::net::TcpListener::bind("localhost:8080").await?;
  // let router = Router::new().route("/", get(paths));
  // // use tracing
  // println!("server operational");

  // axum::serve(listener, router).await?;

  Ok(())
}

// async fn paths() -> &'static str {
//   r#"
// -----------------------------------------------------------------------------------------------------------------------------------------
//         PATH                |           SAMPLE COMMAND
// -----------------------------------------------------------------------------------------------------------------------------------------
// /session: See session data  |  curl -X GET    -H "Content-Type: application/json"                      http://localhost:8080/session
//                             |
// /person/{id}:               |
//   Create a person           |  curl -X POST   -H "Content-Type: application/json" -d '{"name":"John"}' http://localhost:8080/person/one
//   Update a person           |  curl -X PUT    -H "Content-Type: application/json" -d '{"name":"Jane"}' http://localhost:8080/person/one
//   Get a person              |  curl -X GET    -H "Content-Type: application/json"                      http://localhost:8080/person/one
//   Delete a person           |  curl -X DELETE -H "Content-Type: application/json"                      http://localhost:8080/person/one
//                             |
// /people: List all people    |  curl -X GET    -H "Content-Type: application/json"                      http://localhost:8080/people

// /new_user:  Create a new record user
// /new_token: Get instructions for a new token if yours has expired
// "#
// }
 