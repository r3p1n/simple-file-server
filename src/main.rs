use actix_web::{
  HttpServer,
  App,
  middleware,
  web,
  HttpResponse,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenvy::dotenv().ok();

  let version = env!("CARGO_PKG_VERSION");
  let host = std::env::var("HOST").expect("env variable `HOST` is undefined");
  let port: u16 = std::env::var("PORT").expect("env variable `PORT` is undefined")
    .parse().expect("env variable `PORT` is not a number");
  let start_message = format!("v.{} Started on {}:{}", version, host, port);
  println!("{}", start_message);

  let serve_from = std::env::var("CLIENT_DIR_PATH").expect("env variable `CLIENT_DIR_PATH` is undefined");
  let mount_path = std::env::var("CLIENT_PUBLIC_DIR").expect("env variable `CLIENT_PUBLIC_DIR` is undefined");

  HttpServer::new(move || {
    App::new()
      .wrap(actix_cors::Cors::permissive())
      .wrap(middleware::NormalizePath::trim())
      .route("/", web::to(|| async { HttpResponse::Ok().body("Hello! I am a web server!") }))
      .service(actix_files::Files::new(&mount_path, &serve_from)
        // .index_file(&client_index_file)
        // .prefer_utf8(true)
        // .use_last_modified(true)
      )
  })
    .bind((host, port))?
    .run()
    .await
}
