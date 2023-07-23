use crate::components::{Mesh, Velocity, Acceleration};
use nalgebra::Vector3;

pub struct Scene {
    pub meshes: Vec<Mesh>,
    pub velocities: Vec<Velocity>,
    pub accelerations: Vec<Acceleration>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            velocities: Vec::new(),
            accelerations: Vec::new(),
        }
    }
}
