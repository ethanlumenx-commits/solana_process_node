mod logger;
use tracing::info;
fn main() {
    let _guard = logger::init_logger();
    info!("Hello, world!");
}
