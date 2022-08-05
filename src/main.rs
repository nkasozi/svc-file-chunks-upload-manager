mod external;
mod internal;

use crate::internal::web_api::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::run_async().await
}
