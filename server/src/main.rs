use std::{sync::atomic::AtomicU32, mem::MaybeUninit};

use actix_web::{
    get,
    HttpResponse,HttpServer,App,
    web::Data
};
use log::info;
use clap::Parser;
use simple_logger::SimpleLogger;

static counter: AtomicU32 = AtomicU32::new(0);

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Host address
    #[clap(long, default_value = "127.0.0.1")]
    host: String,
    /// Host port
    #[clap(long, default_value = "8081")]
    port: String,
    /// Mongodb Atlas username
    #[clap(long, default_value = "testUser")]
    username: String,
    /// Mongodb Atlas password
    #[clap(long, default_value = "testPassword")]
    password: String,
    /// Mongodb Atlas database name
    #[clap(long, default_value = "classifier")]
    database: String,
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    let args = Args::parse();

    let uri_str = format!(
        "mongodb+srv://{username}:{password}@cluster0.uhxbk.mongodb.net/{database}?retryWrites=true&w=majority",
        username=args.username, password=args.password, database=args.database
    );
    info!("connecting to database: {}", uri_str);
    let client = mongodb::Client::with_uri_str(&uri_str)
        .await
        .expect("could not connect to database");
    let database = client.database(&args.database);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(database.clone()))
            .service(landing)
    })
    .bind(format!("{host}:{port}", host=args.host, port=args.port))?
    .run()
    .await
}

#[get("/")]
async fn landing() -> HttpResponse {
    let a = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    println!("a: {}",a);
    // panic!("hit here");
    HttpResponse::Ok()
        .content_type("application/json")
        .insert_header(("Access-Control-Allow-Origin","*"))
        .body("{ \"a\": 2 }")
}
#[get("/averages")]
async fn user_averages(db_client: Data<mongodb::Database>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

/// Returns the given data serialized to json
#[macro_export]
macro_rules! collection {
    ($args:expr,$client:expr,$collection:expr) => {{
        $client
            .database(&$args.database)
            .collection::<bson::Document>($collection)
    }};
}