use glam::{Mat4, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fov_y: f32, // in radians
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub async fn new(position: Vec3, target: Vec3, aspect: f32) -> Camera {
        Camera {
            position,
            target,
            up: Vec3::Y,
            aspect,
            fov_y: 45.0_f32.to_radians(),
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn set_camera_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = Vec3::new(x, y, z);
    }

    pub fn set_camera_target(&mut self, x: f32, y: f32, z: f32) {
        self.target = Vec3::new(x, y, z);
    }

    pub fn zoom_camera(&mut self, delta: f32) {
        // simple zoom along forward vector
        let forward = (self.target - self.position).normalize();
        self.position += forward * delta;
    }

    // View matrix (world → camera space)
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    // Projection matrix (camera → clip space)
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh_gl(self.fov_y, self.aspect, self.znear, self.zfar)
    }

    // Combined VP matrix
    pub fn view_proj_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
}
