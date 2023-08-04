use glutin;

use glutin::surface::GlSurface;

use crate::camera::Camera;
use crate::components::Mesh;
use crate::entity::Entity;
use crate::input::Keymap;
use crate::renderer::primatives::{Cube, Quad, Vert};
use crate::renderer::{Renderer, texture::Texture};
use crate::scene::Scene;
use crate::shader::Shader;
use crate::windowing::Window;
use nalgebra::{Point3, Point2, };
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
    pub sprite: Texture,
    pub rot: f32,
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
            sprite: Texture::from_path(std::path::Path::new("textures/sprites.png")),
            rot: 0.0,
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
                scale: Scale3::new(0.5, 0.5, 0.5),
            },
        );
        self.player.get_mesh(&mut self.scene).unwrap().verts[0].uv = nalgebra::Point2::new(0.005859, 0.011719);
        self.player.get_mesh(&mut self.scene).unwrap().verts[1].uv = nalgebra::Point2::new(0.039062, 0.011719);
        self.player.get_mesh(&mut self.scene).unwrap().verts[2].uv = nalgebra::Point2::new(0.039062, 0.044922);
        self.player.get_mesh(&mut self.scene).unwrap().verts[3].uv = nalgebra::Point2::new(0.005859, 0.044922);
        self.player
            .add_velocity(&mut self.scene, Vector3::new(0.0, 0.0, 2.0));
        self.player
            .add_acceleration(&mut self.scene, Vector3::new(0.0, 0.0, -0.17));
        self.player
            .add_rot_velocity(&mut self.scene, 1.0 * std::f32::consts::PI / 180.0);
        self.player
            .add_rot_acceleration(&mut self.scene, 0.1 * std::f32::consts::PI / 180.0);
        self.pipes.add_mesh(
            &mut self.scene,
            Mesh {
                verts: vec![
                                 Vert::new(
                                     Point3::new(0.0, -1.0, 0.0-200.0),
                                     Point3::new(0.0, 0.0, 0.0),
                                     Point2::new(0.164062, 0.056641),
                                     Point3::new(0.0, 0.0, 0.0)),
                                 Vert::new(
                                     Point3::new(26.0, -1.0, 0.0-200.0),
                                     Point3::new(0.0, 0.0, 0.0),
                                     Point2::new(0.214844, 0.056641),
                                     Point3::new(0.0, 0.0, 0.0)),
                                 Vert::new(
                                     Point3::new(26.0, -1.0, 160.0-200.0),
                                     Point3::new(0.0, 0.0, 0.0),
                                     Point2::new(0.214844, 0.369141),
                                     Point3::new(0.0, 0.0, 0.0)),
                                 Vert::new(
                                     Point3::new(0.0, -1.0, 160.0-200.0),
                                     Point3::new(0.0, 0.0, 0.0),
                                     Point2::new(0.164062, 0.369141),
                                     Point3::new(0.0, 0.0, 0.0)),
                                 Vert::new(
                                     Point3::new(0.0, -1.0, 0.0+40.0),
                                     Point3::new(0.0, 0.0, 0.0),
                                     Point2::new(0.164062, 0.369141),
                                     Point3::new(0.0, 0.0, 0.0)),
                                 Vert::new(
                                     Point3::new(26.0, -1.0, 0.0+40.0),
                                     Point3::new(0.0, 0.0, 0.0),
                                     Point2::new(0.214844, 0.369141),
                                     Point3::new(0.0, 0.0, 0.0)),
                                 Vert::new(
                                     Point3::new(26.0, -1.0, 160.0+40.0),
                                     Point3::new(0.0, 0.0, 0.0),
                                     Point2::new(0.214844, 0.056641),
                                     Point3::new(0.0, 0.0, 0.0)),
                                 Vert::new(
                                     Point3::new(0.0, -1.0, 160.0+40.0),
                                     Point3::new(0.0, 0.0, 0.0),
                                     Point2::new(0.164062, 0.056641),
                                     Point3::new(0.0, 0.0, 0.0)),
                ],
                elements: vec![0, 1, 2, 0, 2, 3,
                               4, 5, 6, 4, 6, 7],
                translation: Translation3::new(400.0, 0.0, 0.0),
                rotation: Rotation3::new(Vector3::new(0.0, 1.0, 0.0) * 0.0),
                scale: Scale3::new(1.0, 1.0, 1.0),
            },
        );
        self.pipes
            .add_velocity(&mut self.scene, Vector3::new(-48.0, 0.0, 0.0));
    }
    pub fn handle_input(&mut self) {
        if let Some((winit::event::ElementState::Pressed, winit::event::ElementState::Released)) =
            self.keymap.keys.get(&winit::event::VirtualKeyCode::Space)
        {
            self.player
                .get_velocity(&mut self.scene)
                .unwrap()
                .velocity
                .z = 425.0;
            *self.player
                .get_rot_velocity(&mut self.scene)
                .unwrap()
                = 0.0 * std::f32::consts::PI / 180.0;
            self.rot = -45.0 * std::f32::consts::PI / 180.0; 
            self.keymap.keys.insert(winit::event::VirtualKeyCode::Space, (winit::event::ElementState::Released, winit::event::ElementState::Released));
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
        *self.player.get_rot_velocity(&mut self.scene).unwrap() += *self.player.get_rot_acceleration(&mut self.scene).unwrap();
        self.rot += *self.player.get_rot_velocity(&mut self.scene).unwrap() * _dt.as_secs_f32();
        self.rot = self.rot.clamp(-90.0 * std::f32::consts::PI / 180.0, 90.0 * std::f32::consts::PI / 180.0);
        self.player.get_mesh(&mut self.scene).unwrap().rotation = Rotation3::new(Vector3::new(0.0, 1.0, 0.0) * self.rot);

        self.player.get_mesh(&mut self.scene).unwrap().translation.x += self
            .player
            .get_velocity(&mut self.scene)
            .unwrap()
            .velocity
            .x
            * _dt.as_secs_f32();
        self.pipes.get_mesh(&mut self.scene).unwrap().translation.x += self
            .pipes
            .get_velocity(&mut self.scene)
            .unwrap()
            .velocity
            .x
            * _dt.as_secs_f32();

        if self.pipes.get_mesh(&mut self.scene).unwrap().translation.x <= -50.0 {
            self.pipes.get_mesh(&mut self.scene).unwrap().translation.x = 50.0;
        } else if self.pipes.get_mesh(&mut self.scene).unwrap().translation.x >= 50.0 {
            self.pipes.get_mesh(&mut self.scene).unwrap().translation.x = -50.0;
        }

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
            .clamp(-128.0, 128.0);

    }
    pub fn draw(&mut self) {
        self.shader.enable();
        self.shader.set_tex("tex", &self.sprite);
        self.shader.set_mat4("view", self.cam.view());
        self.shader.set_mat4("cam", self.cam.perspective());
        self.renderer.update_meshes(&self.scene.meshes);
        self.renderer.gen_arrays();
        self.renderer.update_buffer();
        self.renderer.newrender(&self.scene);
    }
}
