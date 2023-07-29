use glutin;

use glutin::surface::GlSurface;

use crate::camera::Camera;
use crate::components::Mesh;
use crate::entity::Entity;
use crate::input::Keymap;
use crate::renderer::primatives::{Cube, Quad};
use crate::renderer::Renderer;
use crate::scene::Scene;
use crate::shader::Shader;
use crate::windowing::Window;
use nalgebra::Point3;
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
            cam: Camera::new(Point3::new(0.0, 3.0, 0.0), Point3::new(0.0, 0.0, 0.0)),
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
                verts: Quad::new_square().verts.to_vec(),
                elements: Quad::new_square().elements.to_vec(),
                translation: Translation3::new(0.0, 0.0, 0.0),
                rotation: Rotation3::new(Vector3::new(0.0, 1.0, 1.0) * 0.0),
                scale: Scale3::new(1.0, 1.0, 1.0),
            },
        );
        self.player
            .add_velocity(&mut self.scene, Vector3::new(0.0, 0.0, 2.0));
        self.player
            .add_acceleration(&mut self.scene, Vector3::new(0.0, 0.0, -1.0));
    }
    pub fn handle_input(&mut self) {
        if let Some(winit::event::ElementState::Pressed) =
            self.keymap.keys.get(&winit::event::VirtualKeyCode::Space)
        {
            self.player
                .get_velocity(&mut self.scene)
                .unwrap()
                .velocity
                .z = 50.0;
        }
    }
    pub fn update(&mut self, _dt: std::time::Duration) {
        self.player
            .get_velocity(&mut self.scene)
            .unwrap()
            .velocity
            .x += self
            .player
            .get_acceleration(&mut self.scene)
            .unwrap()
            .acceleration
            .x;
        self.player
            .get_velocity(&mut self.scene)
            .unwrap()
            .velocity
            .y += self
            .player
            .get_acceleration(&mut self.scene)
            .unwrap()
            .acceleration
            .y;
        self.player
            .get_velocity(&mut self.scene)
            .unwrap()
            .velocity
            .z += self
            .player
            .get_acceleration(&mut self.scene)
            .unwrap()
            .acceleration
            .z;

        self.player.get_mesh(&mut self.scene).unwrap().translation.x += self
            .player
            .get_velocity(&mut self.scene)
            .unwrap()
            .velocity
            .x
            * _dt.as_secs_f32();
        self.player.get_mesh(&mut self.scene).unwrap().translation.y += self
            .player
            .get_velocity(&mut self.scene)
            .unwrap()
            .velocity
            .y
            * _dt.as_secs_f32();
        self.player.get_mesh(&mut self.scene).unwrap().translation.z += self
            .player
            .get_velocity(&mut self.scene)
            .unwrap()
            .velocity
            .z
            * _dt.as_secs_f32();

        self.player.get_mesh(&mut self.scene).unwrap().translation.z = self
            .player
            .get_mesh(&mut self.scene)
            .unwrap()
            .translation
            .z
            .clamp(-4.0, 4.0);
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
