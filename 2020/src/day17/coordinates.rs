#[derive(Clone, Debug, PartialEq)]
pub struct Coordinates {
    coordinates: Vec<usize>,
}

pub enum SubResult {
    Overflow,
    Result(Coordinates),
}

impl SubResult {
    #[allow(dead_code)]
    pub fn get_result(&self) -> Coordinates {
        match self {
            SubResult::Overflow => panic!("No result"),
            SubResult::Result(coords) => coords.clone(),
        }
    }
}

impl Coordinates {
    pub fn max_index(&self) -> usize {
        let mut max_index: usize = 1;
        for coord in self.coordinates.iter() {
            max_index *= coord;
        }
        return max_index;
    }

    pub fn index(&self, index: usize) -> &usize {
        &self.coordinates[index]
    }

    pub fn index_mut(&mut self, index: usize) -> &mut usize {
        &mut self.coordinates[index]
    }

    pub fn from(coords: &Vec<usize>) -> Self {
        Self {
            coordinates: coords.clone(),
        }
    }

    pub fn to_index(&self, size: &Coordinates) -> usize {
        let mut index: usize = 0;
        let mut last_offset: usize = 1;
        for coord_index in 0..self.coordinates.len() {
            let coord = self.index(coord_index);
            index += *coord * last_offset;

            let dimension_size = size.index(coord_index);
            last_offset *= *dimension_size;
        }

        return index;
    }

    pub fn from_index(index: usize, size: &Coordinates) -> Self {
        let dimensions = size.coordinates.len();
        let mut last_offset: usize = 1;
        let mut coords: Vec<usize> = Vec::with_capacity(dimensions);

        // 4,4
        // %4, /4
        // 4,3,2
        // %4,/4 %3, /12

        for coord_index in 0..dimensions {
            let dimension_size = *size.index(coord_index);
            coords.push((index / last_offset) % dimension_size);
            last_offset *= dimension_size;
        }

        return Self {
            coordinates: coords,
        }
    }

    pub fn from_ints(coords: &Vec<i32>) -> Self {
        Self {
            coordinates: coords.iter().cloned().map(|n| n as usize).collect(),
        }
    }

    pub fn sub(&self, other: &Vec<i32>, max: &Coordinates) -> SubResult {
        let mut coordinates: Vec<i32> = Vec::with_capacity(self.coordinates.len());
        for (index, coord) in self.coordinates.iter().enumerate() {
            let value = *coord as i32 - other[index];
            if value < 0 {
                return SubResult::Overflow;
            }
            if value >= max.coordinates[index] as i32 {
                return SubResult::Overflow;
            }
            coordinates.push(value);
        }

        let result = Coordinates::from_ints(&coordinates);
        return SubResult::Result(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_coords() -> Coordinates {
        Coordinates::from(&vec![3, 4, 5, 2])
    }

    #[test]
    fn coords_max_index() {
        assert_eq!(3*4*5*2, get_example_coords().max_index());
    }

    #[test]
    fn coords_index_1() {
        assert_eq!(3, *get_example_coords().index(0));
    }

    #[test]
    fn coords_index_2() {
        assert_eq!(5, *get_example_coords().index(2));
    }

    #[test]
    fn coords_to_index_easy() {
        let coords = Coordinates::from(&vec![1, 1, 0, 0]);
        let index = coords.to_index(&get_example_coords());
        
        assert_eq!(1 + 3, index);
    }

    #[test]
    fn coords_to_index_complex() {
        let coords = Coordinates::from(&vec![1, 1, 1, 1]);
        let index = coords.to_index(&get_example_coords());
        
        assert_eq!(1 + 3 + 4 * 3 + 5 * 4 * 3, index);
    }

    #[test]
    fn coords_from_index_easy() {
        let coords = Coordinates::from_index(1 + 3, &get_example_coords());

        assert_eq!(vec![1, 1, 0, 0], coords.coordinates);
    }

    #[test]
    fn coords_from_index_complex() {
        let coords = Coordinates::from_index(1 + 3 + 4 * 3 + 5 * 4 * 3, &get_example_coords());

        assert_eq!(vec![1, 1, 1, 1], coords.coordinates);
    }

    #[test]
    fn coords_sub() {
        let coords = get_example_coords().sub(&vec![2, 3, 4, 1], &Coordinates::from(&vec![2; 4]));

        assert_eq!(vec![1, 1, 1, 1], coords.get_result().coordinates);
    }
}
