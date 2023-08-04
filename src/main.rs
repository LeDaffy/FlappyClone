use flappy;
use glutin;

use glutin::surface::GlSurface;

use flappy::components::Mesh;
use flappy::entity::Entity;
use flappy::input::Keymap;
use flappy::renderer::primatives::Cube;
use flappy::renderer::Renderer;
use flappy::scene::Scene;
use flappy::shader::Shader;
use flappy::windowing::Window;
use nalgebra::{Rotation3, Scale3, Translation3, Vector3};
use winit::{
    self,
    event::{Event, WindowEvent},
};

fn main() {
    let window = flappy::windowing::new().expect("Could not create window");

    let mut flappy = flappy::game::Game::new();
    let mut time = std::time::SystemTime::now();
    let mut d_time = time.elapsed().unwrap();
    flappy.setup();
    window.event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        match event {
            Event::DeviceEvent {
                device_id: _,
                event:
                    winit::event::DeviceEvent::Key(winit::event::KeyboardInput {
                        scancode: _,
                        state: state,
                        virtual_keycode: Some(keycode),
                        modifiers: _,
                    }),
            } => {
                if let Some((current_state, _)) = flappy.keymap.keys.get(&keycode) {
                    flappy.keymap.keys.insert(
                        keycode,
                        (state, *current_state),
                        );
                } else {
                    flappy.keymap.keys.insert(
                        keycode,
                        (state, state),
                        );
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                control_flow.set_exit();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                window.gl_surface.resize(
                    &window.gl_context,
                    std::num::NonZeroU32::new(size.width).unwrap(),
                    std::num::NonZeroU32::new(size.height).unwrap(),
                );
                unsafe {
                    gl::Viewport(0, 0, size.width as i32, size.height as i32);
                }
                flappy.cam.resize(size.width, size.height);
            }
            Event::MainEventsCleared => {
                time = std::time::SystemTime::now();

                flappy.handle_input();
                flappy.update(d_time);
                flappy.draw();

                let _ = window.gl_surface.swap_buffers(&window.gl_context);
                window.window.request_redraw();

                d_time = time.elapsed().unwrap();
            }
            _ => (),
        }
    });
}
