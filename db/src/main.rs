use axum::{Router, routing::get};
use serde::Serialize;
use std::sync::LazyLock;
use surrealdb::{
  Response, Surreal,
  engine::remote::ws::{Client, Ws},
  opt::auth::Root,
};

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[derive(Debug, Serialize)]
struct Person {
  fname: String,
  lname: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // surreal db
  // TODO: use dot envy for logins and ports
  // connect to local db with <Ws> (remote <Wss>)
  DB.connect::<Ws>("0.0.0.0:8000").await?;

  // authenticate with surrealdb
  let auth = DB
    .signin(Root {
      username: "root",
      password: "root",
    })
    .await?;

  // // proper authentication
  //   let token = DB
  //     .signin(Scope {
  //       namespace: "test_namespace",
  //       database: "test_database",
  //       access: "user",
  //       params: Credentials {
  //         username: "root",
  //         pass: "root",
  //       },
  //     })
  //     .await?;
  //   DB.authenticate(token).await?;

  // create the namespace and database
  DB.use_ns("test_namespace").use_db("test_database").await?;

  // setting variables to be used in the db
  DB.set(
    "name",
    Person {
      fname: "noice".to_string(),
      lname: "noie".to_string(),
    },
  )
  .await?;

  // query the variables to the database
  DB.query("CREATE people SET first_person = $name").await?;

  // reponse from the database
  let ress: Response = DB.query("SELECT * FROM people").await?;
  dbg!(ress);

  // // Create a new person with a random id
  // let created: Option<Person> = DB
  //   .create("people")
  //   .content(Person {
  //     name: "noice".to_string(),
  //     age: 13,
  //   })
  //   .await?;
  // dbg!(created);

  // // // Update a person record with a specific id
  // // let updated: Option<Record> = db
  // //   .update(("person", "jaime"))
  // //   .merge(Responsibility { marketing: true })
  // //   .await?;
  // // dbg!(updated);

  // // Select all people records
  // let people: Vec<Person> = DB.select("people").await?;
  // dbg!(people);

  // testing
  println!("db operational");

  // axum server
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
  let router = Router::new().route("/", get(|| async { "Hello, World!" }));
  // use tracing
  println!("server operational");
  axum::serve(listener, router).await?;

  Ok(())
}
