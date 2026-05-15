use why_is_docker_the_best::{tracer, web_server};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracer::init();

    web_server::init().await;
}
