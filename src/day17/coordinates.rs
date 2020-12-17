#[derive(Clone, Debug, PartialEq)]
pub struct Coordinates {

    pub x: usize,
    pub y: usize,
    pub z: usize,
}

pub enum SubResult {
    Overflow,
    Result(Coordinates),
}

impl Coordinates {
    pub fn from(x: usize, y: usize, z: usize) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
        }
    }
    
    pub fn from_ints(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: x as usize,
            y: y as usize,
            z: z as usize,
        }
    }

    pub fn sub(&self, other: &(i32, i32, i32), max: &Coordinates) -> SubResult {
        let x = self.x as i32 - other.0;
        let y = self.y as i32 - other.1;
        let z = self.z as i32 - other.2;
        if x < 0 || y < 0 || z < 0 {
            return SubResult::Overflow;
        }
        if x >= max.x as i32 || y >= max.y as i32 || z >= max.z as i32 {
            return SubResult::Overflow;
        }

        let result = Coordinates::from_ints(x, y, z);
        return SubResult::Result(result);
    }
}
