use nalgebra::{Vector3, Translation3, Rotation3, Scale3};
use crate::renderer::primatives::{Vert};

pub enum Component {
    Mesh(usize),
    Velocity(usize),
    Acceleration(usize),
}


#[derive(Debug)]
pub struct Mesh {
    pub verts: Vec<Vert>,
    pub elements: Vec<u32>,
    pub translation: Translation3<f32>,
    pub rotation: Rotation3<f32>,
    pub scale: Scale3<f32>,
}

impl Mesh {
    pub fn verts(&self) -> Vec<Vert> {
        self.verts.clone()
    }

    pub fn verts_transformed(&self) -> Vec<Vert> {
        self.verts.clone()
            .into_iter()
            .map(|mut v| {
                v.pos = self.rotation * v.pos;
                v.pos = self.scale * v.pos;
                v.pos = self.translation * v.pos;
                v
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct Velocity {
    pub velocity: Vector3<f32>,
}
#[derive(Debug)]
pub struct Acceleration {
    pub acceleration: Vector3<f32>,
}
