use crate::state::State;
use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, ElementState, KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct App<'a> {
    pub window: Option<Arc<Window>>,
    pub state: Option<State<'a>>,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window = Arc::new(
                event_loop
                    .create_window(Window::default_attributes())
                    .unwrap(),
            );
            self.window = Some(window.clone());

            let state = pollster::block_on(State::new(window.clone()));
            self.state = Some(state);
        }
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let mut last_render_time = Instant::now();
        if window_id == self.state.as_ref().expect("you are not stated").window.id() {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    let now = Instant::now();
                    let dt = now - last_render_time;
                    last_render_time = now;
                    let state = self
                        .state
                        .as_mut()
                        .expect("you must state before you do things");
                    state.update(dt);
                    match state.render() {
                        Ok(_) => {}

                        Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                WindowEvent::Resized(physical_size) => self
                    .state
                    .as_mut()
                    .expect("you dont have a state yet dumbass")
                    .resize(physical_size),
                _ => (),
            }
        }
        let now = Instant::now();
        let dt = now - last_render_time;
        last_render_time = now;

        self.state.as_mut().expect("aaaaa").input(&event);
        self.state.as_mut().expect("bbbbb").update(dt);
    }
    fn device_event(&mut self, event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::Added => {}
            DeviceEvent::Removed => {}
            DeviceEvent::MouseMotion {delta, } => if self.state.as_mut().unwrap().mouse_pressed{
                self.state.as_mut().unwrap().camera_controller.process_mouse(delta.0, delta.1)
            }
            DeviceEvent::MouseWheel { .. } => {}
            DeviceEvent::Motion { .. } => {}
            DeviceEvent::Button { .. } => {}
            DeviceEvent::Key(_) => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.state
            .as_mut()
            .expect("kys or fix the damn state")
            .window()
            .request_redraw();
    }
}