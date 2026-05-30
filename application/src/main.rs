use application::Application;
use macroquad::prelude::*;

#[macroquad::main("SandFalls")]
async fn main() {
    set_default_filter_mode(FilterMode::Nearest);
    let mut application = Application::new();
    application.run().await;
}
