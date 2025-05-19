use axum::{Router, routing::get};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::any::connect, opt::auth::Root};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
  fname: String,
  lname: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // surreal db
  // connect to local db with <Ws> (remote <Wss>)
  // db.connect::<Ws>("DB_PORT").await?;
  // using surreakv for embedded database
  let db = connect("surrealkv://db").await?;

  // authenticate with surrealdb
  db.signin(Root {
    username: "root",
    password: "root",
  })
  .await?;

  // create the namespace and database
  db.use_ns("test_namespace").use_db("test_database").await?;

  // posting records from the database
  // FIX: add type annotations without assigning a variable
  let rec: Option<Person> = db
    .create(("table_name", 1))
    .content(Person {
      fname: "foo bar".to_string(),
      lname: "bar foo".to_string(),
    })
    .await?;
  // reading records from the database
  // vec for entire record response
  let res: Vec<Person> = db.select("table_name").await?;
  dbg!(&res);
  // opt for specific record
  let ress: Option<Person> = db.select(("table_name", 1)).await?;
  dbg!(&ress);

  // // updating records from the database
  // let record: Option<Person> = DB
  //   .update(("table_name", 0))
  //   .content(Person {
  //     fname: "foo bar".to_string(),
  //     lname: "bar foo".to_string(),
  //   })
  //   .await?;

  // // reading records from the database
  // // vec for entire record response
  // let res: Vec<Person> = DB.select("table_name").await?;
  // dbg!(res);
  // // opt for specific record
  // let ress: Option<Person> = DB.select(("table_name", 0)).await?;
  // dbg!(ress);

  // // delete records and entire tables on the database
  // // // vec for entire record response
  // let del: Vec<Person> = DB.delete("table_name").await?;
  // dbg!(del);
  // // opt for specific record
  // let dell: Option<Person> = DB.delete(("table_name", "0")).await?;
  // dbg!(dell);

  // testing
  println!("db connected");

  // axum server
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
  let router = Router::new().route("/", get(move || async { "hello world!" }));
  println!("server operational");
  axum::serve(listener, router).await?;

  Ok(())
}
