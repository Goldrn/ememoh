use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::{Window, WindowId};
use crate::state::{Pipeline, State};

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
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if window_id == self.state.as_ref().expect("you are not stated").window.id() {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::KeyboardInput {
                    device_id,
                    event: ref event_key,
                    is_synthetic,
                } => {
                    if event_key.clone().physical_key == KeyCode::Space && event_key.state.clone().is_pressed() {
                        self.state
                            .as_mut()
                            .expect("kys or fix the damn state")
                            .window()
                            .request_redraw();
                        println!("eat shit and die");
                    }
                }
                WindowEvent::RedrawRequested => {
                    let state = self
                        .state
                        .as_mut()
                        .expect("you must state before you do things");
                    state.update();
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
        self.state.as_mut().expect("aaaaa").input(&event);
        self.state.as_mut().expect("bbbbb").update();
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.state
            .as_mut()
            .expect("kys or fix the damn state")
            .window()
            .request_redraw();
    }
}