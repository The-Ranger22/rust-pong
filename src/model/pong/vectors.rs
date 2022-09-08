use core::fmt;
use std::{ops, fmt::{Display, Formatter}};


#[derive(Debug, Clone, Copy)]
pub struct EuclideanVector {
    magnitude: f64,
    angle: f64
}

impl Display for EuclideanVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.magnitude, self.angle)
    }
}

impl PartialEq for EuclideanVector {
    fn eq(&self, other: &Self) -> bool {
        self.get_angle() == other.get_angle() && self.get_magnitude() == other.get_magnitude()
    }
}

impl Eq for EuclideanVector {}

impl EuclideanVector {
    pub fn new(magnitude: f64, angle: f64) -> Self {
        Self {
            magnitude,
            angle
        }
    }

    pub fn get_angle(&self) -> f64 {
        self.angle
    }

    pub fn get_magnitude(&self) -> f64 {
        self.magnitude
    }

    pub fn set_angle(&mut self, angle: f64) {
        self.angle = angle;
    }

    pub fn set_magnitude(&mut self, magnitude: f64) {
        self.magnitude = magnitude
    }
}

impl EuclideanVector {
    fn _normalize(u: f64, magnitude: f64) -> f64 {
        u/magnitude
    }
}

impl EuclideanVector {

    fn calculate_angle(x_comp: f64, y_comp: f64) -> f64 {
        y_comp.atan2(x_comp).to_degrees()
    }

    pub fn x_component(&self) -> f64 {
        self.magnitude * self.angle.to_radians().cos()
    }

    pub fn y_component(&self) -> f64 {
        self.magnitude * self.angle.to_radians().sin()
    }

    pub fn normal_x_component(&self) -> f64 {
        Self::_normalize(self.x_component(), self.get_magnitude())
    }

    pub fn normal_y_component(&self) -> f64 {
        Self::_normalize(self.y_component(), self.get_magnitude())
    }

    pub fn normalized(&self) -> Self {
        Self::from_components(self.normal_x_component(), self.normal_y_component())
    }

    pub fn from_components(x_comp: f64, y_comp: f64) -> Self {
        let magnitude = (x_comp.powi(2) + y_comp.powi(2)).sqrt();
        let angle = Self::calculate_angle(x_comp, y_comp);
        Self {
            magnitude,
            angle
        }
    }
}

// implementing dot product
impl ops::Mul<EuclideanVector> for EuclideanVector {
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.magnitude * rhs.magnitude * ((self.angle - rhs.angle).abs().to_radians().cos())
    }
}

// scalar multiplication
impl ops::Mul<f64> for EuclideanVector {
    type Output = EuclideanVector;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.magnitude*rhs, self.angle)
    }
}
// addition
impl ops::Add<EuclideanVector> for EuclideanVector {
    type Output = EuclideanVector;
    fn add(self, rhs: EuclideanVector) -> Self::Output {
        Self::from_components(
            self.x_component() + rhs.x_component(),
            self.y_component() + rhs.y_component()
        )
    }
}

// subtraction
impl ops::Sub<EuclideanVector> for EuclideanVector {
    type Output = EuclideanVector;
    fn sub(self, rhs: EuclideanVector) -> Self::Output {
        Self::from_components(
            self.x_component() - rhs.x_component(),
            self.y_component() - rhs.y_component() 
        )
    }
}

impl EuclideanVector {
    
    pub fn update(&mut self, new_vec: EuclideanVector) {
        self.magnitude = new_vec.magnitude;
        self.angle = new_vec.angle;
    }

    pub fn collide_with(&mut self, opposing_vector: EuclideanVector) {
        let v = self.normalized();
        let opposing_vector = opposing_vector.normalized();
        let u = opposing_vector*((v * opposing_vector) / (opposing_vector*opposing_vector));
        let w = v - u;
        self.set_angle((w - u).get_angle());
    }
}
#[cfg(test)]
mod tests {
    use crate::model::pong::vectors::EuclideanVector;

    #[test]
    fn test_bed() {
        let mut v1 = EuclideanVector::new(10.0, 45.0);
        let v2 = EuclideanVector::new(1.0, 180.0);

        let mut norm_v1 = v1.normalized();
        let norm_v2 = v2.normalized();
        println!("{} | {}", v1, norm_v1);
        v1.collide_with(v2);
        norm_v1.collide_with(norm_v2);

        println!("{} | {}", v1, norm_v1);
        println!("{} {} | {} {}", v1.x_component(), v1.y_component(), norm_v1.x_component(), norm_v1.y_component())
    }

    #[test]
    fn fn_new_control() {
        let new_vec = EuclideanVector::new(0.0, 0.0);
        assert_eq!(new_vec, EuclideanVector{magnitude: 0.0, angle: 0.0})
    }
}