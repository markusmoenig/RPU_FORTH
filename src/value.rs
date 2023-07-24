use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Value {
    Number(f32),
    Shape3D(SDF3D),
    Array(Vec<Value>)
}

pub use Value::*;

impl Value {

    /// Converts the value to a number
    pub fn to_number(&self) -> Option<f32> {
        match self {
            Number(v) => {
                Some(*v)
            },
            _ => {
                None
            }
        }
    }

    /// Converts the value to a color index
    pub fn to_color(&self) -> Option<u8> {
        match self {
            Number(v) => {
                Some(*v as u8)
            },
            _ => {
                None
            }
        }
    }

    /// Converts the value to a String
    pub fn to_string(&self) -> String{
        match self {
            Number(v) => {
                format!("{:}", v)
            },
            Shape3D(sdf) => {
                sdf.to_string()
            },
            Array(values)=> {
                let mut s = "[".to_string();
                for v in values {
                    s += v.to_string().as_str();
                }
                s += "]";
                s.to_string()
            }
        }
    }
}