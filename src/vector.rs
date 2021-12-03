extern crate rogue_sdl;
use std::{ops};
pub struct Vector2D{
    pub x: f64,
    pub y: f64,
}

impl ops::Add<Vector2D> for Vector2D {//enables + operator for this struct
    type Output = Vector2D;
    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {x: self.x + other.x, y: self.y+other.y}
    }
}
impl ops::Sub<Vector2D> for Vector2D {// enables -
    type Output = Vector2D;
    fn sub(self, other: Vector2D) -> Vector2D {
        Vector2D {x: self.x - other.x, y: self.y - other.y}
    }
}
impl ops::Sub<f64> for Vector2D {// enables -
    type Output = Vector2D;
    fn sub(self, other: f64) -> Vector2D {
        Vector2D {x: self.x - other, y: self.y - other}
    }
}
impl ops::Add<f64> for Vector2D {// enables -
    type Output = Vector2D;
    fn add(self, other: f64) -> Vector2D {
        Vector2D {x: self.x + other, y: self.y + other}
    }
}
impl ops::Div<Vector2D> for Vector2D{//enables /
    type Output = Vector2D;
    fn div(self, other: Vector2D)-> Vector2D{
        Vector2D{x: self.x / other.x, y: self.y / other.y}
    }
}
impl ops::Div<Vector2D> for f64{
    type Output = Vector2D;
    fn div(self, other:Vector2D)->Vector2D{
        Vector2D{x: self / other.x, y: self / other.y}

    }
}
impl ops::Mul<Vector2D> for f64{
    type Output = Vector2D;
    fn mul(self, other:Vector2D)->Vector2D{
        Vector2D{x: self * other.x, y: self * other.y}
    }
}
impl ops::Mul<Vector2D> for Vector2D{//enables dot product
    type Output = f64;
    fn mul(self, other: Vector2D)-> f64{
         self.x * other.x + self.y * other.y
    }
}
impl ops::Mul<f64> for Vector2D{//enables * with scalars
    type Output = Vector2D;
    fn mul(self, other: f64)-> Vector2D{
        Vector2D{x: self.x * other, y: self.y * other}

    }
}
impl ops::Div<f64> for Vector2D{//enables / with scalars 
    type Output = Vector2D;
    fn div(self, other:f64)-> Vector2D{
        Vector2D{x: self.x / other, y: self.y / other}
    }
}
impl ops::Neg for Vector2D{//get the negative of the vector
    type Output = Vector2D;
    fn neg(self)->Vector2D{
        Vector2D{x: -self.x, y: -self.y}
    }
}
impl Copy for Vector2D { }

impl Clone for Vector2D {
    fn clone(&self) -> Vector2D {
        *self
    }
}
impl PartialEq for Vector2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[allow(dead_code)]
impl Vector2D{
    pub fn new(x:f64,y: f64)-> Vector2D{
        Vector2D{x,y}
    }
    pub fn length(self) -> f64{
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn length_squared(self) -> f64{
        self.x * self.x + self.y * self.y
    }
   pub fn distance(self,  other: Vector2D) -> f64{
        let delta_v = self-other;
        (delta_v.x * delta_v.x + delta_v.y * delta_v.y).sqrt()
    }
    pub fn normalize(self) -> Vector2D{
        self / self.length()
    }
   pub fn cross(self, other:Vector2D) -> f64{
        self.x * other.y - self.y * self.x
    }
}