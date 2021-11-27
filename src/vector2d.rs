struct Vector2D{
    x: f64,
    y: f64,
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
impl ops::Div<Vector2D> for Vector2D{//enables /
    type Output = Vector2D;
    fn div(self, other: Vector2D)-> Vector2D{
        Vector2D{x: self.x / other.x, y: self.y / other.y}
    }
}
impl ops::Mul<Vector2D> for Vector2D{//enables *
    type Output = Vector2D;
    fn mul(self, other: Vector2D)-> Vector2D{
        Vector2D{x: self.x * other.x, y: self.y * other.y}
    }
}
impl ops::Neg for Vector2D{
    type Output = Vector2D;
    fn neg(self){
        Vector2D{x: -self.x, y: -self.y}
    }
}