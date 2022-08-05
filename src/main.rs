mod external;
mod internal;

use crate::internal::api::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::run_async().await
}
