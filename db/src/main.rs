use axum::{Router, routing::get};
use std::sync::LazyLock;
use surrealdb::{
  Surreal,
  engine::remote::ws::{Client, Ws},
  opt::auth::Root,
};

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // surreal db
  // TODO: create container with logins for surreal
  // TODO: use dot envy for logins and ports
  // connect to local db with <Ws> (remote <Wss>)
  DB.connect::<Ws>("0.0.0.0:8000").await?;

  // authenticate
  DB.signin(Root {
    username: "root",
    password: "root",
  })
  .await?;

  // create the namespace and database
  DB.use_ns("test_namespace").use_db("test_database").await?;

  // TODO: write queries on a .suql
  // set some queries
  let sql = "
  CREATE website:article;
  SET topic=13, time=13, best=13;
  ";

  // getting the data
  let setup: Vec<u8> = DB.select("website").await?;

  // testing
  println!("db operational");
  println!("{:?}", setup);

  // axum server
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
  let router = Router::new().route("/", get(|| async { "Hello, World!" }));
  // use tracing
  println!("server operational");
  axum::serve(listener, router).await?;

  Ok(())
}
