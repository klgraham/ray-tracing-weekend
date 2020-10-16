use std::cmp::PartialEq;
use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};
use rand::prelude::*;

// geom.rs

/// A point in 3D Euclidean space
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

/// A 3D vector in Euclidean space
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}


impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3 {x, y, z}
    }

    pub fn origin() -> Self {
        Point3::new(0.0, 0.0, 0.0)
    }
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 {x, y, z}
    }

    /// Returns the $L^2$ norm
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Returns length squared
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns a unit vector
    pub fn to_unit_vector(self) -> Self {
        let length = self.norm();
        Vector3::new(self.x / length,
                    self.y / length,
                    self.z / length)
    }

    /// Dot product
    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product
    // cyclic permutations are positive
    pub fn cross(&self, other: &Vector3) -> Self {
        let (x, y, z) = (other.x, other.y, other.z);
        Vector3::new(
            self.y * &z - self.z * &y,
            self.z * &x - self.x * &z,
            self.x * &y - self.y * &x
        )
    }

    /// Reflects vector against a surface with normal vector `n`
    pub fn reflect(&self, n: &Vector3) -> Vector3 {
        return *self - (2.0 * self.dot(n)) * (*n);
    }

    /// Refraction via Snell's law
    pub fn refract(&self, n: &Vector3, etai_over_etat: f64) -> Vector3 {
        let cos_theta = (-(*self)).dot(n);
        let r_out_perpendicular: Vector3 = etai_over_etat * (*self + cos_theta * (*n));
        let r_out_parallel: Vector3 = (1.0 - r_out_perpendicular.length_squared()).abs().sqrt() * -(*n);
        return r_out_perpendicular + r_out_parallel;

    }
}


/// Returns a random point inside the unit sphere
pub fn random_point_in_unit_sphere() -> Vector3 {
    let mut rng = rand::thread_rng();
    loop {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        let z: f64 = rng.gen();
        let v = Vector3::new(x, y, z);

        if v.length_squared() >= 1.0 {
            continue;
        }
        return v
    }
}

pub fn random_unit_vector() -> Vector3 {
    let mut rng = rand::thread_rng();
    let phi:f64 = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
    let z:f64 = rng.gen_range(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vector3::new(r * phi.cos(), r * phi.sin(), z)
}

pub fn random_in_hemisphere(normal: &Vector3) -> Vector3 {
    let in_unit_sphere = random_point_in_unit_sphere();
    if normal.dot(&in_unit_sphere) > 0.0 {
        // random vector in same hemisphere as normal
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}


/// Point + Vector addition
impl Add<Vector3> for Point3 {
    type Output = Point3;

    fn add(self, other: Vector3) -> Point3 {
        Point3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

/// Point + Vector addition assignment
impl AddAssign<Vector3> for Point3 {
    fn add_assign(&mut self, other: Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

/// Vector addition
impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

/// Vector addition assignment
impl AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

/// Subtracting a `Point` from a `Point`
impl Sub<Point3> for Point3 {
    type Output = Vector3;

    fn sub(self, other: Point3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

/// Subtracting a `Vector` from a `Point`
impl Sub<Vector3> for Point3 {
    type Output = Point3;

    fn sub(self, other: Vector3) -> Point3 {
        Point3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

/// Subtracting a `Vector` from a `Point`, with assignment
impl SubAssign<Vector3> for Point3 {
    fn sub_assign(&mut self, other: Vector3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

/// `Vector` Subtraction
impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

/// Subtracting a `Vector` from a `Vector`, with assignment
impl SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

/// Negating a `Point`
impl Neg for Point3 {
    type Output = Point3;

    fn neg(self) -> Point3 {
        Point3::new(-self.x, -self.y, -self.z)
    }
}

/// Negating a `Vector`
impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

/// Scalar multiplication for Points
impl Mul<f64> for Point3 {
    type Output = Point3;

    fn mul(self, a: f64) -> Point3 {
        Point3::new(self.x * a, self.y * a, self.z * a)
    }
}

/// Scalar multiplication for Points
impl Mul<Point3> for f64 {
    type Output = Point3;

    fn mul(self, other: Point3) -> Point3 {
        Point3::new(other.x * self, other.y * self, other.z * self)
    }
}

/// Scalar multiplication for Vector
impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, a: f64) -> Vector3 {
        Vector3::new(self.x * a, self.y * a, self.z * a)
    }
}

/// Elementwise multiplication for Vector (Hadamard product)
impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

/// Scalar multiplication for Vector
impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3::new(other.x * self, other.y * self, other.z * self)
    }
}

/// Scalar multiplication for vector, with assignment
impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, a: f64) {
        self.x *= a;        
        self.y *= a;        
        self.z *= a;        
    }
}

/// Scalar division for Point
impl Div<f64> for Point3 {
    type Output = Point3;

    fn div(self, other: f64) -> Point3 {
        (1.0 / other) * self
    }
}

/// Scalar division for Vector
impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f64) -> Vector3 {
        (1.0 / other) * self
    }
}

/// Scalar division for vector, with assignment
impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, a: f64) {
        self.x /= a;        
        self.y /= a;        
        self.z /= a;       
    }
}


#[cfg(test)]
mod tests {
    use super::{Point3, Vector3};

    #[test]
    fn can_add_tuples() {
        let p = Point3::new(3.0, -2.0, 5.0);
        let v = Vector3::new(-2.0, 3.0, 1.0);
        assert_eq!(p + v, Point3::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn can_subtract_points() {
        let p1 = Point3::new(3.0, 2.0, 1.0);
        let p2 = Point3::new(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Vector3::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn can_subtract_a_vector_from_a_point() {
        let p = Point3::new(3.0, 2.0, 1.0);
        let v = Vector3::new(5.0, 6.0, 7.0);
        assert_eq!(p - v, Point3::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn can_subtract_vectors() {
        let v1 = Vector3::new(3.0, 2.0, 1.0);
        let v2 = Vector3::new(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, Vector3::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn can_negate_tuples() {
        let p = Point3::new(3.0, -2.0, 5.0);
        let v = Vector3::new(-2.0, 3.0, 1.0);
        assert_eq!(-p, Point3::new(-3.0, 2.0, -5.0));
        assert_eq!(-v, Vector3::new(2.0, -3.0, -1.0));
    }

    #[test]
    fn can_multiply_tuple_by_scalar() {
        let p = Point3::new(3.0, -2.0, 5.0);
        let v = Vector3::new(-2.0, 3.0, 1.0);
        let a = 0.5f64;
        assert_eq!(a * p, Point3::new(1.5, -1.0, 2.5));
        assert_eq!(a * v, Vector3::new(-1.0, 1.5, 0.5));
    }

    #[test]
    fn can_divide_tuple_by_scalar() {
        let p = Point3::new(3.0, -2.0, 5.0);
        let v = Vector3::new(-2.0, 3.0, 1.0);
        let a = 2.0f64;
        assert_eq!(p / a, Point3::new(1.5, -1.0, 2.5));
        assert_eq!(v / a, Vector3::new(-1.0, 1.5, 0.5));
    }

    #[test]
    fn can_compute_vector_norm() {
        let v = Vector3::new(1.0, 2.0, 3f64.sqrt());
        assert_eq!(v.norm(), 2f64 * 2f64.sqrt());
    }

    #[test]
    fn can_convert_vector_to_unit_vector() {
        let v = Vector3::new(4.0, 0.0, 0.0);
        assert_eq!(v.to_unit_vector(), Vector3::new(1.0, 0.0, 0.0));

        let v = Vector3::new(1.0, 2.0, 3.0);
        let unit = v.to_unit_vector();
        let diff = (unit.norm() - 1_f64).abs();
        assert!(diff <= 0.00001_f64);
    }

    #[test]
    fn can_compute_dot_prod() {
        let u = Vector3::new(1.0, 2.0, 3.0);
        let v = Vector3::new(2.0, 3.0, 4.0);
        assert_eq!(u.dot(v), 20f64);
    }

    #[test]
    fn can_compute_cross_prod() {
        let u = Vector3::new(1.0, 2.0, 3.0);
        let v = Vector3::new(2.0, 3.0, 4.0);
        assert_eq!(u.cross(v), Vector3::new(-1.0, 2.0, -1.0));
        assert_eq!(v.cross(u), Vector3::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn compute_hadamard_product() {
        let u = Vector3::new(1.0, 0.2, 0.4);
        let v = Vector3::new(0.9, 1.0, 0.50);
        assert_eq!(u * v, Vector3::new(0.9, 0.2, 0.2));
    }
}