use axum::{Router, routing::get};
use std::sync::LazyLock;
use surrealdb::{
  Surreal,
  engine::remote::ws::{Client, Ws},
};

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // surreal db
  // connect to local db with <Ws> (remote <Wss>)
  DB.connect::<Ws>("0.0.0.0:8000").await?;
  // create the namespace and database
  DB.use_ns("test_namespace").use_db("test_database").await?;
  // set some queries
  let sql = "
  CREATE website:article;
  SET topic='hello world', time=13, best=false;
  ";
  let setup: Vec<_> = DB.select("website").await?;
  // testing
  println!("db operational {:?}", setup);

  // axum server
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
  let router = Router::new().route("/", get(|| async { "Hello, World!" }));
  // use tracing
  println!("server operational");
  axum::serve(listener, router).await?;

  Ok(())
}
