fn main() {
    println!("{:?}", Point::from((1.0, 2.0, 3.0)));
    println!("{:?}", Vector::from((1.0, 2.0, 3.0)));
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl From<(f32, f32, f32)> for Point {
    fn from(value: (f32, f32, f32)) -> Self {
        Point {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[derive(Debug)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl From<(f32, f32, f32)> for Vector {
    fn from(value: (f32, f32, f32)) -> Self {
        Vector {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn create_point_from_tuple() {
        assert_eq!(
            Point::from((1.0, 2.0, 3.0)),
            Point {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn create_vector_from_tuple() {
        assert_eq!(
            Vector::from((1.0, 2.0, 3.0)),
            Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }
}
