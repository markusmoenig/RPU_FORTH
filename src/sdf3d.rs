use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum SDF3DType {
    Box,
    Sphere
}

use SDF3DType::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SDF3D {
    sdf_type                    : SDF3DType,


    radius                      : f32,
    size                        : Vec3f,
    textures                    : Vec<Value>
}

impl SDF3D {
    pub fn new(sdf_type: SDF3DType) -> Self {

        Self {
            sdf_type,

            radius              : 0.5,
            size                : Vec3f::zero(),

            textures            : vec![Value::Number(10.0)],
        }
    }

    /// Return the distance to the SDF
    pub fn distance(&self, p: Vec3f, position: Vec3f) -> f32 {

        // fn op_rep_lim(p: Vec3f, s: f32, lima: Vec3f, limb: Vec3f ) -> Vec3f {
        //     return p-s*clamp(round(p/s),lima,limb);
        // }

        let mut d: f32 = std::f32::MAX;

    	//vec2 q = p*6.0 - vec2(5.0,0.0);
        //vec2 r = opRepLim(q,2.0,vec2(-1,-2),vec2(1,2));

        //p = op_rep_lim(p, 0.25, vec3f(-1.0, 0.0, -0.0), vec3f(1.0, 2.0, 0.0));

        if self.sdf_type == Box {
            let q = abs(p - position) - self.size;
            d = length(max(q,Vec3f::new(0.0, 0.0, 0.0))) + min(max(q.x,q.y),0.0);
        } else
        if self.sdf_type == Sphere {
            d = length(p - position) - self.radius;
        }
        d
    }

    /// Read the properties of the SDF from the stack.
    pub fn read_properties(&mut self, stack:  &mut Vec<Value>) -> Result<(), String> {

        if self.sdf_type == Box {
            if let Some(v) = stack.pop() {
                if let Some(n) = v.to_number() {
                    self.size.z = n;
                } else {
                    return Err("Wrong value on stack. Expected number for \"depth\" of Box.".into());
                }
            } else {
                return Err("Stack is empty. Expected number for \"depth\" of Box.".into());
            }
            if let Some(v) = stack.pop() {
                if let Some(n) = v.to_number() {
                    self.size.y = n;
                } else {
                    return Err("Wrong value on stack. Expected number for \"height\" of Box.".into());
                }
            } else {
                return Err("Stack is empty. Expected number for \"height\" of Box.".into());
            }
            if let Some(v) = stack.pop() {
                if let Some(n) = v.to_number() {
                    self.size.x = n;
                } else {
                    return Err("Wrong value on stack. Expected number for \"width\" of Box.".into());
                }
            } else {
                return Err("Stack is empty. Expected number for \"width\" of Box.".into());
            }
        } else
        if self.sdf_type == Sphere {
            if let Some(v) = stack.pop() {
                if let Some(n) = v.to_number() {
                    self.radius = n;
                } else {
                    return Err("Wrong value on stack. Expected number for \"radius\" of Sphere.".into());
                }
            } else {
                return Err("Stack is empty. Expected number for \"radius\" of Sphere.".into());
            }
        }

        loop {
            if let Some(option) = stack.last() {
                match option {
                    Value::Array(values) => {
                        self.textures = values.clone();
                        _ = stack.pop();
                    },
                    _ => {
                        break;
                    }
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    /// Gets a random color index
    pub fn get_color(&self) -> u8 {
        if let Some(color) = self.textures[0].to_color() {
            color
        } else {
            0
        }
    }

    /// Generates a bounding box for the SDF
    pub fn create_bbox(&self, position: Vec3f) -> AABB {
        let mut min: Vec3<f32> = Vec3f::zero();
        let mut max: Vec3<f32> = Vec3f::zero();

        if self.sdf_type == Box {
            min = position - self.size;
            max = position + self.size;
        } else
        if self.sdf_type == Sphere {
            min = position - self.radius;
            max = position + self.radius;
        }

        AABB { min, max }
    }

    pub fn to_string(&self) -> String {
        match self.sdf_type {
            Sphere => {
                "Sphere".into()
            },
            Box => {
                "Box".into()
            }
        }
    }

    /*
    pub fn apply(&self, key: Vec3i, tile_key: Vec3i) {

        let mut world = WORLD.lock().unwrap();

        if let Some(mut tile) = world.get_tile(key) {

            tile.set_voxel(10, 10, 10, Some((10, 10)));

            let size = tile.size;

            //let start_y = 0;//record.tile_key.y + 1;
            //let height = 1;

            let hp = Vec3f::from(tile_key);


            for y in 0..size {
                for x in 0..size {
                    for z in 0..size {
                        let p = vec3f(x as f32, y as f32, z as f32);
                        // if length(p - hp) - size as f32 / 3.0 < 0.0 {
                        //     //tile.set(vec3i(x, y, z), "Color", "Material");
                        //         tile.set_voxel(x, y, z, Some((10, 10)));
                        // }

                        let p = p - hp; let b = vec3f(10.0, 10.0, 10.0);
                        let q = abs(p) - b;
                        let d = length(max(q,vec3f(0.0, 0.0, 0.0))) + min(max(q.x,max(q.y,q.z)),0.0);
                        if d < 0.0 {
                                tile.set_voxel(x, y, z, Some((10, 10)));
                        }
                    }
                }
            }

            world.set_tile(key, tile);
        }
    }

    pub fn create_preview(&self, pixels: &mut [u8], rect: Rect, stride: usize) {

        let half = rect.width as f32 / 2.0;

        #[inline(always)]
        pub fn mix(a: &f32, b: &f32, v: f32) -> f32 {
            (1.0 - v) * a + b * v
        }

        fn shade(d: f32) -> [u8;4] {
            let dist = d*100.0;
            let banding = max(sin(dist), 0.0);
            let strength = sqrt(1.0-exp(-abs(d)*0.2));
            let pattern = mix(&strength, &banding, (0.6-abs(strength-0.5))*0.3);
            let mut c = if d > 0.0 { vec3f(0.0,0.0,0.0) } else { vec3f(0.9,0.9,0.9) };
            c *= pattern;

            [(c.x * 255.0) as u8, (c.y * 255.0) as u8, (c.z * 255.0) as u8, 255]
        }

        if self.sdf_type == Box {
            let size = half - 5.0;
            for y in rect.y..rect.y + rect.height {
                for x in rect.x..rect.x + rect.width {
                    let i = x * 4 + y * stride * 4;

                    let q = abs(vec2f(x as f32 - rect.x as f32, y as f32 - rect.y as f32) - vec2f(half, half)) - vec2f(size, size);
                    let d = length(max(q,Vec2f::new(0.0, 0.0))) + min(max(q.x,q.y),0.0);

                    pixels[i..i + 4].copy_from_slice(&shade(d));
                }
            }
        } else
        if self.sdf_type == Sphere {
            let size = half - 5.0;
            for y in rect.y..rect.y + rect.height {
                for x in rect.x..rect.x + rect.width {
                    let i = x * 4 + y * stride * 4;
                    let d = length(vec2f(x as f32 - rect.x as f32, y as f32 - rect.y as f32) - vec2f(half, half)) - size;
                    pixels[i..i + 4].copy_from_slice(&shade(d));
                }
            }
        }

    }*/
}
