use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Value {
    Number(f32),
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

    /// Converts the value to a String
    pub fn to_string(&self) -> String{
        match self {
            Number(v) => {
                format!("{:}", v)
            }
        }
    }
}