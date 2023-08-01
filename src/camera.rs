use nalgebra::{self, Point3, Vector3, Matrix4, Rotation3};
pub struct Camera {
    position: Point3<f32>,
    look_at: Point3<f32>,
    look_at_right: Point3<f32>,
    view: Matrix4<f32>,
    perspective: Matrix4<f32>,
    width: u32,
    height: u32,
}

impl Camera {
    pub fn new(position: Point3<f32>, look_at: Point3<f32>) -> Self {
        Self {
            position,
            look_at,
            look_at_right: Point3::new(1.0, 0.0, 0.0),
            view: Matrix4::look_at_rh(&position, &look_at, &Vector3::new(0.0, 0.0, 1.0)),
            //perspective: Matrix4::new_perspective(16.0/9.0, 90.0, 0.01, 1000.0),
            perspective: Matrix4::new_orthographic(-4.0, 4.0, -4.0, 4.0, -0.01, 100.0),
            width: 1920,
            height: 1080,
        }
    }
    pub fn rotate_ver(&mut self, mut angle: f32) {
        angle = angle * std::f32::consts::PI / 180.0;
        let axis  = self.look_at_right - self.look_at;
        //println!("{:?}", axis);
        self.position = Rotation3::from_axis_angle(&nalgebra::UnitVector3::new_normalize(Vector3::new(axis.x, axis.y, axis.z)), angle) * self.position;
        self.view =Matrix4::look_at_rh(&self.position, &self.look_at, &Vector3::new(0.0, 0.0, 1.0));

    }
    pub fn rotate_hor(&mut self, mut angle: f32) {
        angle = angle * std::f32::consts::PI / 180.0;
        //println!("{:?}", self.look_at_right);
        self.position = Rotation3::from_euler_angles(0.0, 0.0, angle) * self.position ;
        self.look_at_right = Rotation3::from_euler_angles(0.0, 0.0, angle) * self.look_at_right;
        self.view =Matrix4::look_at_rh(&self.position, &self.look_at, &Vector3::new(0.0, 0.0, 1.0));

    }
    pub fn resize(&mut self, width: u32, height: u32) {
        self.perspective = Matrix4::new_perspective((width as f32)/(height as f32), 90.0, 0.01, 1000.0);
        let zoom = 5.0;
            self.perspective = Matrix4::new_orthographic((width as f32 / -zoom), (width as f32 / zoom), (height as f32 / -zoom),(height as f32 / zoom), 0.01, 100.0);
        self.width = width;
        self.height = height;
    }
    pub fn view(&self) -> &Matrix4<f32> { 
        &self.view
    }
    pub fn perspective(&self) -> &Matrix4<f32> { 
        &self.perspective
    }
    pub fn pan(&mut self, offset: &nalgebra::Translation3<f32>) {
        self.position = offset * self.position;
        self.look_at = offset * self.position;
        self.look_at_right = offset * self.position;
        self.view =Matrix4::look_at_rh(&self.position, &self.look_at, &Vector3::new(0.0, 0.0, 1.0));
    }
    pub fn set_position(mut self, position: &Point3<f32>) {
        self.position = *position;
        self.view =Matrix4::look_at_rh(&self.position, &self.look_at, &Vector3::new(0.0, 0.0, 1.0));
    }
}
