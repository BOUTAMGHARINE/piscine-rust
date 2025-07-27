use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);   


impl Point {
    pub fn distance(&self,other:Point)->f64{
        ((self.0-other.0).powi(2)+(self.1-other.1).powi(2)).sqrt()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x1:f64,x2:f64,r:f64)->Self{
        Self{
            center:Point(x1,x2),
            radius:r,
        }
    }
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }

    pub fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    pub fn intersect(&self, other: Circle) -> bool {
        let distance = self.center.distance(other.center);
        distance < (self.radius + other.radius)
    }
}
