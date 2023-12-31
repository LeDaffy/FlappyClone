use crate::components::{Acceleration, Mesh, Velocity};
use crate::scene::Scene;
use nalgebra::Vector3;

pub struct Entity{
    pub mesh: Option<usize>,
    pub velocity: Option<usize>,
    pub acceleration: Option<usize>,
    pub rot_velocity: Option<usize>,
    pub rot_acceleration: Option<usize>,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            mesh: None,
            velocity: None,
            acceleration: None,
            rot_velocity: None,
            rot_acceleration: None,
        }
    }
    pub fn add_mesh(&mut self, scene: &mut Scene, mesh: Mesh) {
        scene.meshes.push(mesh);
        self.mesh = Some(scene.meshes.len() - 1);
    }
    pub fn get_mesh<'a>(&self, scene: &'a mut Scene) -> Option<&'a mut Mesh> {
        if let Some(idx) = self.mesh {
            return Some(&mut scene.meshes[idx]);
        }

        None
    }
    pub fn get_mesh_index(&self) -> Option<usize> {
        if let Some(idx) = self.mesh {
            return Some(idx);
        }
        None
    }
    pub fn add_velocity(&mut self, scene: &mut Scene, velocity: Vector3<f32>) {
        let vel = Velocity {velocity: velocity};
        scene.velocities.push(vel);
        self.velocity = Some(scene.velocities.len() - 1);
    }
    pub fn get_velocity<'a>(&self, scene: &'a mut Scene) -> Option<&'a mut Velocity> {
        if let Some(idx) = self.velocity {
            return Some(&mut scene.velocities[idx]);
        }

        None
    }
    pub fn get_velocity_index(&self) -> Option<usize> {
        if let Some(idx) = self.velocity {
            return Some(idx);
        }
        None
    }
    pub fn add_rot_velocity(&mut self, scene: &mut Scene, velocity: f32) {
        scene.rot_velocities.push(velocity);
        self.rot_velocity = Some(scene.velocities.len() - 1);
    }
    pub fn get_rot_velocity<'a>(&self, scene: &'a mut Scene) -> Option<&'a mut f32> {
        if let Some(idx) = self.rot_velocity {
            return Some(&mut scene.rot_velocities[idx]);
        }
        None
    }
    pub fn get_rot_velocity_index(&self) -> Option<usize> {
        if let Some(idx) = self.rot_velocity {
            return Some(idx);
        }
        None
    }
    pub fn add_rot_acceleration(&mut self, scene: &mut Scene, acceleration: f32) {
        scene.rot_accelerations.push(acceleration);
        self.rot_acceleration = Some(scene.accelerations.len() - 1);
    }
    pub fn get_rot_acceleration<'a>(&self, scene: &'a mut Scene) -> Option<&'a mut f32> {
        if let Some(idx) = self.rot_acceleration {
            return Some(&mut scene.rot_accelerations[idx]);
        }
        None
    }
    pub fn get_rot_acceleration_index(&self) -> Option<usize> {
        if let Some(idx) = self.rot_acceleration {
            return Some(idx);
        }
        None
    }
    pub fn add_acceleration(&mut self, scene: &mut Scene, acceleration: Vector3<f32>) {
        let acc = Acceleration {acceleration: acceleration};
        scene.accelerations.push(acc);
        self.acceleration = Some(scene.accelerations.len() - 1);
    }
    pub fn get_acceleration<'a>(&self, scene: &'a mut Scene) -> Option<&'a mut Acceleration> {
        if let Some(idx) = self.acceleration {
            return Some(&mut scene.accelerations[idx]);
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
