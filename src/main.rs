#[macro_use]
extern crate approx;

use nalgebra;

fn main() {
}

type Point = nalgebra::Point3<f32>;
type Vector = nalgebra::Vector3<f32>;

impl Point {
    fn neg(self) -> Point {
        let origin = Point::new(0.0,0.0,0.0);
        return origin - self
    }
}

fn point(a: f32,b:f32,c:f32) -> Point {
    return Point::new(a,b,c)
}

fn vector(a: f32,b:f32,c:f32) -> Vector {
    return Vector::new(a,b,c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_point() {
        assert_eq!(
            point(1.0,2.0,3.0),
            Point::new(1.0,2.0,3.0)
        );
    }

    #[test]
    fn create_a_vector() {
        assert_eq!(
            vector(1.0,2.0,3.0),
            Vector::new(1.0,2.0,3.0)
        );
    }

    #[test]
    fn subtract_two_points() {
        let p1 = point(1.1,2.2,3.3);
        let p2 = point(1.1,1.1,1.1);
        assert_relative_eq!(p1-p2, vector(0.0,1.1,2.2))
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = point(1.1,2.2,3.3);
        let v = vector(1.1,1.1,1.1);

        assert_relative_eq!(p-v, point(0.0,1.1,2.2))
    }

    #[test]
    fn subtract_two_vectors() {
        let v1 = vector(1.1,2.2,3.3);
        let v2 = vector(1.1,0.0,1.1);
        assert_relative_eq!(v1-v2, vector(0.0,2.2,2.2))
    }
}
