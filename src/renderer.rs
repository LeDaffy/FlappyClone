pub mod primatives;
pub mod texture;
use crate::scene::Scene;

use gl::{self, types::*};

use crate::camera::Camera;
use crate::components;
use crate::renderer::texture::Texture;
use crate::shader::Shader;
use nalgebra::{self, Matrix4, Point3, Vector3};
use primatives::Vert;

pub struct Renderer {
    pub vao: GLuint,
    pub vbo: GLuint,
    pub ebo: GLuint,
    pub verts: Vec<Vert>,
    pub elements: Vec<u32>,
    pub shader: Option<Shader>,
    pub camera: Camera,
    pub tex: Texture,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            vao: 0,
            vbo: 0,
            ebo: 0,
            verts: Vec::new(),
            elements: Vec::new(),
            shader: None,
            camera: Camera::new(Point3::new(0.0, 2.0, 1.0), Point3::new(0.0, 0.0, 0.0)),
            tex: Texture::from_path(std::path::Path::new("textures/container.png")),
        }
    }
    pub fn shader_from_paths(&mut self, vs_path: &std::path::Path, fs_path: &std::path::Path) {
        self.shader = Some(Shader::new(vs_path, fs_path));
    }
    pub fn set_shader(&mut self, shader: Shader) {
        self.shader = Some(shader);
    }

    pub fn update_meshes(&mut self, meshes: &Vec<components::Mesh>) {
        self.verts = meshes
            .iter()
            .map(|m| m.verts_transformed())
            .flatten()
            .collect();
        let mut offset: u32 = 0;
        self.elements = meshes
            .iter()
            .map(|m| {
                let x: Vec<u32> = m
                    .elements
                    .clone()
                    .into_iter()
                    .map(|mut e| {
                        e += offset;
                        e
                    })
                    .collect();
                offset += m.verts.len() as u32;
                x
            })
            .flatten()
            .collect();
    }

    pub fn update_buffer(&self) {
        let vertices = self
            .verts
            .iter()
            .map(|v| {
                [
                    v.pos.x, v.pos.y, v.pos.z, v.color.x, v.color.y, v.color.z, v.uv.x, v.uv.y,
                    v.normal.x, v.normal.y, v.normal.z,
                ]
            })
            .flatten()
            .collect::<Vec<f32>>();
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const std::ffi::c_void,
            );
        }
    }

    pub fn gen_arrays(&mut self) {
        let vertices = self
            .verts
            .iter()
            .map(|v| {
                [
                    v.pos.x, v.pos.y, v.pos.z, v.color.x, v.color.y, v.color.z, v.uv.x, v.uv.y,
                    v.normal.x, v.normal.y, v.normal.z,
                ]
            })
            .flatten()
            .collect::<Vec<f32>>();
        static mut ONCE: bool = true;
        unsafe {
            if ONCE {
                println!(
                    "{:?}",
                    self.verts
                        .iter()
                        .map(|v| [v.pos.x, v.pos.y, v.pos.z])
                        .flatten()
                        .collect::<Vec<f32>>()
                );
                println!("{:?}", self.elements);
                println!("{:?}", self.elements.len());
                ONCE = false;
            }
        }

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);

            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const std::ffi::c_void,
                gl::DYNAMIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.elements.len() * std::mem::size_of::<u32>()) as isize,
                self.elements.as_ptr() as *const std::ffi::c_void,
                gl::DYNAMIC_DRAW,
            );

            // enable position
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (11 * std::mem::size_of::<f32>()) as i32,
                (0 * std::mem::size_of::<f32>()) as *const std::ffi::c_void,
            );
            gl::EnableVertexAttribArray(0);

            // enable color
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (11 * std::mem::size_of::<f32>()) as i32,
                (3 * std::mem::size_of::<f32>()) as *const std::ffi::c_void,
            );
            gl::EnableVertexAttribArray(1);

            // enable uv
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                (11 * std::mem::size_of::<f32>()) as i32,
                (6 * std::mem::size_of::<f32>()) as *const std::ffi::c_void,
            );
            gl::EnableVertexAttribArray(2);

            // enable surface normal
            gl::VertexAttribPointer(
                3,
                3,
                gl::FLOAT,
                gl::FALSE,
                (11 * std::mem::size_of::<f32>()) as i32,
                (8 * std::mem::size_of::<f32>()) as *const std::ffi::c_void,
            );
            gl::EnableVertexAttribArray(3);

            // note that this is allowed, the call to glVertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
            // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
            gl::BindVertexArray(0);
        }
    }
    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vao); // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
            self.update_buffer();
            // draw our first triangle
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            //gl::DrawArrays(gl::TRIANGLES, 0, num_verts as i32);
            //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::DrawElements(
                gl::TRIANGLES,
                self.elements.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
    pub fn newrender(&mut self, scene: &Scene) {
        unsafe {
            gl::BindVertexArray(self.vao); // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::DrawElements(
                gl::TRIANGLES,
                self.elements.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}
