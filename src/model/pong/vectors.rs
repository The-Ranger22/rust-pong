use ndarray::arr1;
use core::fmt;
use std::{ops, fmt::{Display, Formatter}};


enum Quadrant {
    I,
    II,
    III,
    IV
}

impl Quadrant {
    fn determine_quadrant(x: f64, y: f64) -> Quadrant {
        if x.is_sign_positive() && y.is_sign_positive() {
            Self::I
        } else if x.is_sign_negative() && y.is_sign_positive() {
            Self::II
        } else if x.is_sign_negative() && y.is_sign_negative() {
            Self::III
        } else {
            Self::IV
        }
    }

    fn angle_offset(&self) -> f64 {
        match self {
            Quadrant::I     => 0.0,
            Quadrant::II    => 90.0,
            Quadrant::III   => 180.0,
            Quadrant::IV    => 270.0,
        }
    }
}

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

impl EuclideanVector {
    pub fn new(magnitude: f64, angle: f64) -> Self {
        Self {
            magnitude,
            angle
        }
    }
}


impl EuclideanVector {

    fn calculate_angle(x_comp: f64, y_comp: f64) -> f64 {
        let quad = Quadrant::determine_quadrant(x_comp, y_comp);
        (y_comp.abs()/x_comp.abs()).atan().to_degrees().round() + quad.angle_offset()
    }

    pub fn x_component(&self) -> f64 {
        self.magnitude * self.angle.to_radians().cos()
    }

    pub fn y_component(&self) -> f64 {
        self.magnitude * self.angle.to_radians().sin().round()
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
        let v = *self;
        let u = opposing_vector*((v * opposing_vector) / (opposing_vector*opposing_vector));
        let w = v - u;
        let new_vec = w - u;
        self.update(new_vec);
    }
}