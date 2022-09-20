use std::path::Path;

use rocket::{
    fs::{relative, FileServer, NamedFile},
    launch, routes, tokio::sync::broadcast::channel, get,
};

#[launch]
/// Start the web server
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("web")))
        .mount("/", routes!(wasm))
}

#[get("/wasm")]
async fn wasm() -> Option<NamedFile> {
    NamedFile::open(Path::new("wasm/pkg/acquire_rs_wasm_bg.wasm"))
        .await
        .ok()
}