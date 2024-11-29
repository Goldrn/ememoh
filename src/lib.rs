mod state;
mod app;
mod objects;
mod camera;
mod math;
mod constants;

mod cameracontroller;
mod model;
mod loader;

use crate::app::App;

use winit::{
    event_loop::EventLoop,
};
pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    let mut app = App::default();
    event_loop.run_app(&mut app).expect("window failed to open");
}
