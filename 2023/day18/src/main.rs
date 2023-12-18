fn main() {
    let input = get_input();
    let result_part1 = part1(input);
    println!("Part1: {}", result_part1);
}

fn part1(input: &str) -> usize {
    let plan = Plan::from(input);
    let polygon = plan.construct_circumference();
    let area = area_of_polygon(&polygon);
    let permimeter = perimeter_of_polygon(&polygon);
    let area_inside = area + 1 - permimeter / 2;
    let area_for_real = permimeter + area_inside;
    return area_for_real;
}

fn area_of_polygon(polygon: &Vec<Position>) -> usize {
    let mut sum = 0;
    for (index, point2) in polygon.iter().enumerate().skip(1) {
        let point1 = polygon.get(index - 1).unwrap();
        sum += (point1.y + point2.y) * (point1.x - point2.x);
    }
    let area = sum.abs() as usize / 2;
    return area;
}

fn perimeter_of_polygon(polygon: &Vec<Position>) -> usize {
    let mut sum = 0;
    for (index, point1) in polygon.iter().enumerate() {
        let point2 = polygon.get((index + 1) % polygon.len()).unwrap();
        sum += point1.x.abs_diff(point2.x) + point1.y.abs_diff(point2.y);
    }

    return sum;
}

struct Plan {
    digs: Vec<Dig>,
}

impl Plan {
    fn construct_circumference(&self) -> Vec<Position> {
        let mut polygon = Vec::new();
        let mut last_position = Position { x: 0, y: 0 };
        for dig in self.digs.iter() {
            last_position = last_position.move_by(dig);
            polygon.push(last_position)
        }
        return polygon;
    }

    fn from(input: &str) -> Self {
        let digs = input.lines().map(|line| Dig::from(line)).collect();
        let result = Plan { digs };
        return result;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn times(&mut self, times: isize) {
        self.x *= times;
        self.y *= times;
    }

    fn move_by(&self, dig: &Dig) -> Self {
        let mut position = dig.direction.move_vector();
        position.times(dig.length);
        position.x += self.x;
        position.y += self.y;
        return position;
    }
}

struct Dig {
    length: isize,
    direction: Direction,
}

impl Dig {
    fn from(str: &str) -> Self {
        let parts: Vec<&str> = str.split(' ').collect();
        let direction = Direction::from(parts[0].chars().next().unwrap()).unwrap();
        let length = parts[1].parse().unwrap();
        let result = Self { length, direction };
        return result;
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn move_vector(&self) -> Position {
        match self {
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Down => Position { x: 0, y: 1 },
        }
    }

    fn from(char: char) -> Option<Self> {
        match char {
            'L' => Some(Self::Left),
            'R' => Some(Self::Right),
            'U' => Some(Self::Up),
            'D' => Some(Self::Down),
            _ => None,
        }
    }
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        include_str!("./example.txt")
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = part1(input);
        assert_eq!(result, 62);
    }
}
