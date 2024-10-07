use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::camera::Camera;
use crate::math::Vector3;

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub(crate) fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub(crate) fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                KeyEvent {
                    state,
                    physical_key: PhysicalKey::Code(keycode),
                    ..
                },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {KeyCode::KeyW | KeyCode::ArrowUp => {
                    self.is_forward_pressed = is_pressed;
                    true
                }
                    KeyCode::KeyA | KeyCode::ArrowLeft => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyS | KeyCode::ArrowDown => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyD | KeyCode::ArrowRight => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub(crate) fn update_camera(&self, camera: &mut Camera) {
        let forward_fucked = camera.target.clone() - camera.eye.clone();
        let forward = Vector3::new(forward_fucked.x, forward_fucked.y, forward_fucked.z);
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when the camera gets too close to the
        // center of the scene.
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye = camera.eye.clone() + forward_norm.clone() * self.speed;
            println!("forward");
        }
        if self.is_backward_pressed {
            camera.eye = camera.eye.clone() - forward_norm.clone() * self.speed;
            println!("backward");
        }

        let right = forward_norm.cross(camera.up.clone());

        // Redo radius calc in case the forward/backward is pressed.
        let forward = camera.target.clone() - camera.eye.clone();
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            // Rescale the distance between the target and the eye so
            // that it doesn't change. The eye, therefore, still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target.clone() - (forward.clone() + right.clone() * self.speed).normalize() * forward_mag;
            println!("right");
        }
        if self.is_left_pressed {
            camera.eye = camera.target.clone() - (forward.clone() - right.clone() * self.speed).normalize() * forward_mag;
            println!("left");
        }
    }
}
