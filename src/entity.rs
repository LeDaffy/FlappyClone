use crate::components::{Acceleration, Mesh, Velocity};
use crate::scene::Scene;
use nalgebra::Vector3;

pub struct Entity{
    pub mesh: Option<usize>,
    pub velocity: Option<usize>,
    pub acceleration: Option<usize>,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            mesh: None,
            velocity: None,
            acceleration: None,
        }
    }
    pub fn add_mesh(&mut self, scene: &mut Scene, mesh: Mesh) {
        scene.meshes.push(mesh);
        self.mesh = Some(scene.meshes.len() - 1);
    }
    pub fn get_mesh<'a>(&self, scene: &'a Scene) -> Option<&'a Mesh> {
        if let Some(idx) = self.mesh {
            return Some(&scene.meshes[idx]);
        }

        None
    }
    pub fn get_mesh_index(&self) -> Option<usize> {
        if let Some(idx) = self.mesh {
            return Some(idx);
        }
        None
    }
    pub fn get_velocity<'a>(&self, scene: &'a Scene) -> Option<&'a Velocity> {
        if let Some(idx) = self.velocity {
            return Some(&scene.velocities[idx]);
        }

        None
    }
    pub fn get_velocity_index(&self) -> Option<usize> {
        if let Some(idx) = self.velocity {
            return Some(idx);
        }
        None
    }
    pub fn get_acceleration<'a>(&self, scene: &'a Scene) -> Option<&'a Acceleration> {
        if let Some(idx) = self.acceleration {
            return Some(&scene.accelerations[idx]);
        }

        None
    }
    pub fn get_acceleration_index(&self) -> Option<usize> {
        if let Some(idx) = self.acceleration {
            return Some(idx);
        }
        None
    }
}
