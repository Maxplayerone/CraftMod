#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Self {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn normalize(&mut self) -> Vec3{
        let mag = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec3{
        x: self.x / mag,
        y: self.y / mag,
        z: self.z / mag,
        }
    }
}

#[derive(Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn normalize(&mut self){
        let mag = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        self.x = self.x / mag;
        self.y = self.y / mag;
        self.z = self.z / mag;
        self.w = self.w / mag;
    }
}

pub struct Mat4{
    pub mat: [f32; 16],
}

impl Mat4{
    pub fn new(value: f32) -> Mat4{
        Self{
            mat: [
            value, 0.0, 0.0, 0.0,
            0.0, value, 0.0, 0.0,
            0.0, 0.0, value, 0.0,
            0.0, 0.0, 0.0, 1.0,
            ]
        }
    }

    pub fn translate(&mut self, vec: Vec3){
        self.mat[12] = vec.x;
        self.mat[13] = vec.y;
        self.mat[14] = vec.z;
    }

    pub fn rotate(&mut self, axis_normalized: Vec3, angle_degrees: f32){
        let angle_radians = angle_degrees * (3.1415 / 180.0);
        let s = angle_radians.sin();
        let c = angle_radians.cos();
        let one_minus_c = 1.0 - c;

        self.mat[0] = one_minus_c * axis_normalized.x * axis_normalized.x + c;
        self.mat[1] = one_minus_c * axis_normalized.x * axis_normalized.y + s * axis_normalized.z;
        self.mat[2] = one_minus_c * axis_normalized.x * axis_normalized.z - s * axis_normalized.y;

        self.mat[4] = one_minus_c * axis_normalized.x * axis_normalized.y - s * axis_normalized.z;
        self.mat[5] = one_minus_c * axis_normalized.y * axis_normalized.y + c;
        self.mat[6] = one_minus_c * axis_normalized.y * axis_normalized.z + s * axis_normalized.x;

        self.mat[8] = one_minus_c * axis_normalized.x * axis_normalized.z + s * axis_normalized.y;
        self.mat[9] = one_minus_c * axis_normalized.y * axis_normalized.z - s * axis_normalized.x;
        self.mat[10] = one_minus_c * axis_normalized.z * axis_normalized.z + c;
    }

    
        pub fn perspective(&mut self, fov: f32, aspect_ratio: f32, near: f32, far: f32){
            //asserts
            let f = (fov / 2.0).tan().recip();
            self.mat[0] = f / aspect_ratio;
            self.mat[5] = f;
            self.mat[10] = (far + near) / (near - far);
            self.mat[14] = (2.0 * far * near) / (near - far); 
        }
}