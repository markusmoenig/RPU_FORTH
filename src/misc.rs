use crate::prelude::*;

/*
vec2 hash2(vec3 p3) {
	p3 = fract(p3 * vec3(5.3983, 5.4427, 6.9371));
    p3 += dot(p3, p3.yzx + 19.19);
    return fract((p3.xx + p3.yz) * p3.zy);
}*/

pub fn hash3_2(mut p3: Vec3f) -> Vec2f {
    p3 = frac(p3 * Vec3f::new(5.3983, 5.4427, 6.9371 ));
    p3 += dot(p3, p3.yzx() + 19.19);
    frac((p3.xx() + p3.yz()) * p3.zy())
}

/// AABB
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AABB {
    pub min             : Vec3f,
    pub max             : Vec3f,
}

impl AABB {
    pub fn get_size(&self) -> Vec3f {
        self.max - self.min
    }
}

use std::ops::Index;

impl Index<usize> for AABB {
    type Output = Vec3f;

    fn index(&self, index: usize) -> &Vec3f {
        if index == 0 {
            &self.min
        } else {
            &self.max
        }
    }
}

/// Ray
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Ray {
    pub o                   : Vec3f,
    pub d                   : Vec3f,

    pub inv_direction       : Vec3f,

    pub sign_x              : usize,
    pub sign_y              : usize,
    pub sign_z              : usize,
}

impl Ray {

    pub fn new(o : Vec3f, d : Vec3f) -> Self {
        Self {
            o,
            d,

            inv_direction   : Vec3f::new(1.0 / d.x, 1.0 / d.y, 1.0 / d.z),
            sign_x          : (d.x < 0.0) as usize,
            sign_y          : (d.y < 0.0) as usize,
            sign_z          : (d.z < 0.0) as usize
        }
    }

    /// Returns the position on the ray at the given distance
    pub fn at(&self, d: f32) -> Vec3f {
        self.o + self.d * d
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back
}

/// HitRecord
#[derive(PartialEq, Debug, Clone)]
pub struct HitRecord {
    pub hitpoint        : Vec3f,
    pub key             : Vec3i,
    pub tile_key        : Vec3i,
    pub distance        : f32,
    pub normal          : Vec3f,
    pub uv              : Vec3f,
    pub value           : (u8, u8),
    pub side            : Side,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            hitpoint    : Vec3f::zero(),
            key         : Vec3i::zero(),
            tile_key    : Vec3i::zero(),
            distance    : 0.0,
            normal      : Vec3f::zero(),
            uv          : Vec3f::zero(),
            value       : (0, 0),
            side        : Side::Top,
        }
    }

    pub fn compute_side(&mut self) {
        if self.normal.y > 0.5 {
            self.side = Side::Bottom;
        } else
        if self.normal.y < -0.5 {
            self.side = Side::Top;
        } else
        if self.normal.x > 0.5 {
            self.side = Side::Left;
        } else
        if self.normal.x < -0.5 {
            self.side = Side::Right;
        } else
        if self.normal.z > 0.5 {
            self.side = Side::Back;
        } else
        if self.normal.z < -0.5 {
            self.side = Side::Front;
        }
    }

    pub fn get_side(&mut self) -> Side {
        self.side.clone()
    }

}

/// Location
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Location {
    BackLeft,
    BackMiddle,
    BackRight,
    MiddleLeft,
    Middle,
    FrontLeft,
    FrontMiddle,
    FrongRight,
}

/// BAKE
pub struct Bake {
    pub sdf             : Option<SDF3D>,

    pub location        : Location,
    pub facing          : Side,
}

impl Bake {
    pub fn new() -> Self {
        Self {
            sdf         : None,
            location    : Location::FrontLeft,
            facing      : Side::Front,
        }
    }
}