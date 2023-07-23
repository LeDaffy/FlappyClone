use glutin;

use glutin::surface::GlSurface;

use nalgebra::Point3;
use crate::components::Mesh;
use crate::entity::Entity;
use crate::input::Keymap;
use crate::renderer::primatives::Cube;
use crate::renderer::Renderer;
use crate::scene::Scene;
use crate::shader::Shader;
use crate::camera::Camera;
use crate::windowing::Window;
use nalgebra::{Rotation3, Scale3, Translation3, Vector3};
use winit::{
    self,
    event::{Event, WindowEvent},
};

pub struct Game {
    pub scene: Scene,
    pub renderer: Renderer,
    pub cam: Camera,
    pub keymap: Keymap,
    pub player: Entity,
    pub pipes: Entity,
    pub shader: Shader,
}

impl Game {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            renderer: Renderer::new(),
            cam: Camera::new(Point3::new(0.0, 3.0, 1.0), 
                                Point3::new(0.0, 0.0, 0.0)),
            keymap: Keymap::new(),
            player: Entity::new(),
            pipes: Entity::new(),
            shader: Shader::new(
                std::path::Path::new("shaders/hello.vs"),
                std::path::Path::new("shaders/hello.fs"),
            ),
        }
    }
    pub fn setup(&mut self) {
        self.player.add_mesh(
            &mut self.scene,
            Mesh {
                verts: Cube::new().verts().to_vec(),
                elements: Cube::new().elements(),
                translation: Translation3::new(0.0, 0.0, 0.0),
                rotation: Rotation3::new(Vector3::new(0.0, 1.0, 1.0) * 0.0),
                scale: Scale3::new(1.0, 1.0, 1.0),
            },
        );
    }
    pub fn handle_input(&self) {
        if let Some(winit::event::ElementState::Pressed) =
            self.keymap.keys.get(&winit::event::VirtualKeyCode::Space)
        {
                    println!("Space");
        }
    }
    pub fn update(&self, _dt: std::time::Duration) {

    }
    pub fn draw(&mut self) {
        self.shader.enable();
        self.shader.set_mat4("view", self.cam.view());
        self.shader.set_mat4("cam", self.cam.perspective());
        self.renderer.update_meshes(&self.scene.meshes);
        self.renderer.gen_arrays();
        self.renderer.update_buffer();
        self.renderer.newrender(&self.scene);
    }
}
