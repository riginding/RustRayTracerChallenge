use nalgebra;

type Point = nalgebra::Point3<f32>;
type Vector = nalgebra::Vector3<f32>;

fn point(a: f32,b:f32,c:f32) -> Point {
    Point::new(a,b,c)
}

fn vector(a: f32,b:f32,c:f32) -> Vector {
    Vector::new(a,b,c)
}

fn magnitude(vec: Vector) -> f32 {
    nalgebra::Matrix::norm(&vec)
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

    #[test]
    fn negate_vector() {
        let v1 = vector(1.1, -2.2, 3.3);
        assert_relative_eq!(-v1, vector(-1.1, 2.2, -3.3))
    }

    #[test]
    fn multiply_with_scalar() {
        let v1 = vector(1.1, -2.2, 3.3);
        assert_relative_eq!(3.0 * v1 , vector(3.3, -6.6, 9.9))
    }

    #[test]
    fn divide_by_scalar() {
        let v1 = vector(9.9, 6.6, 3.3);
        assert_relative_eq!(v1 / 3.0 , vector(3.3, 2.2, 1.1))
    }

    #[test]
    fn magnitude_of_vector() {
        let v1 = vector(9.9, 6.6, 3.3);
        assert_relative_eq!(v1 / 3.0 , vector(3.3, 2.2, 1.1))
    }

    #[test]
    fn magnitude_of_vec1() {
        let v1 = vector(1.,0.,0.);
        assert_relative_eq!(magnitude(v1), 1.)
    }

    #[test]
    fn magnitude_of_vec2() {
        let v1 = vector(0.,1.,0.);
        assert_relative_eq!(magnitude(v1), 1.)
    }

    #[test]
    fn magnitude_of_vec3() {
        let v1 = vector(0.,0.,1.);
        assert_relative_eq!(magnitude(v1), 1.)
    }

    #[test]
    fn magnitude_of_vec4() {
        let v1 = vector(1.,2.,3.);
        let result = 14.0_f32;
        assert_relative_eq!(magnitude(v1), result.sqrt())
    }

    #[test]
    fn magnitude_of_vec5() {
        let v1 = vector(-1.,-2.,-3.);
        let result = 14.0_f32;
        assert_relative_eq!(magnitude(v1), result.sqrt())
    }
}
