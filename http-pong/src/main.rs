use actix_files::Files;
use actix_files::NamedFile;
use actix_web::rt;
use actix_web::{
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder, middleware,
    web,
};
use actix_ws::AggregatedMessage;
use env_logger::Env;
use mblistener::ButtonState;
use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::channel;

mod mblistener;

struct AppData {
    sender: Sender<ButtonState>,
}

enum SelectResult {
    WebsocketMessage(AggregatedMessage),
    BroadcastMessage(ButtonState),
    Error(anyhow::Error),
}

async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppData>,
) -> Result<HttpResponse, Error> {
    todo!()
}

async fn index() -> impl Responder {
    NamedFile::open_async("./public/index.html").await.unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (tx, _rx) = channel::<ButtonState>(1_000_000);
    let tx_clone = tx.clone();

    env_logger::init_from_env(Env::new());

    rt::spawn(async move {
        if let Err(e) = mblistener::start(tx_clone).await {
            println!("Encountered error while benchmarking: {e}");
        }
    });

    HttpServer::new(move || {
        let tx_clone = tx.clone();

        App::new()
            .app_data(web::Data::new(AppData { sender: tx_clone }))
            .service(web::resource("/").to(index))
            .service(Files::new("/public", "./public").prefer_utf8(true))
            .service(
                web::resource("/websocket").route(web::get().to(websocket)),
            )
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
