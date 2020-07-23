use messages_actix::MessageApp;
use std::io::Result;

fn main() -> std::io::Result<_> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    MessageApp::new(8080)
        .run();
}

