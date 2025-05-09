use axum::{Router, routing::get};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use surrealdb::{
  Surreal,
  engine::remote::ws::{Client, Ws},
  opt::auth::Root,
};

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[derive(Debug, Serialize, Deserialize)]
struct Person {
  fname: String,
  lname: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // surreal db
  // TODO: use dot envy for logins and ports
  // TODO: using tracing to log
  // connect to local db with <Ws> (remote <Wss>)
  DB.connect::<Ws>("0.0.0.0:8000").await?;

  // authenticate with surrealdb
  DB.signin(Root {
    username: "root",
    password: "root",
  })
  .await?;

  // create the namespace and database
  DB.use_ns("test_namespace").use_db("test_database").await?;

  // posting records from the database
  // FIX: add type annotations without assigning a variable
  let record: Option<Person> = DB
    .create(("table_name", 0))
    .content(Person {
      fname: "foo".to_string(),
      lname: "bar".to_string(),
    })
    .await?;

  // reading records from the database
  // vec for entire record response
  let res: Vec<Person> = DB.select("table_name").await?;
  dbg!(res);
  // opt for specific record
  let ress: Option<Person> = DB.select(("table_name", 0)).await?;
  dbg!(ress);

  // updating records from the database
  let record: Option<Person> = DB
    .update(("table_name", 0))
    .content(Person {
      fname: "foo bar".to_string(),
      lname: "bar foo".to_string(),
    })
    .await?;

  // reading records from the database
  // vec for entire record response
  let res: Vec<Person> = DB.select("table_name").await?;
  dbg!(res);
  // opt for specific record
  let ress: Option<Person> = DB.select(("table_name", 0)).await?;
  dbg!(ress);

  // delete records and entire tables on the database
  // // vec for entire record response
  let del: Vec<Person> = DB.delete("table_name").await?;
  dbg!(del);
  // opt for specific record
  let dell: Option<Person> = DB.delete(("table_name", "0")).await?;
  dbg!(dell);

  // testing
  println!("db connected");

  // axum server
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
  let router = Router::new().route("/", get(|| async { "Hello, World!" }));
  // use tracing
  println!("server operational");
  axum::serve(listener, router).await?;

  Ok(())
}
