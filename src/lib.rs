mod texture;
mod state;
mod app;
mod vertex;
mod objects;
mod camera;
mod math;

use std::{ops::Not, process::Output, ptr::null, sync::Arc};

use tokio::runtime::Runtime;
use wgpu::{
    hal::{Label, Surface},
    util::DeviceExt,
    TextureDimension,
};
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, KeyCode, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
};
use crate::app::App;
pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    let mut app = App::default();
    event_loop.run_app(&mut app).expect("window failed to open");
}
