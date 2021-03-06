extern crate rogue_sdl;


use std::f64;
use sdl2::rect::Rect;
use sdl2::rect::Point;

use crate::gamedata::TILE_SIZE_PROJECTILE;
use crate::vector::*;

#[derive(Copy,Clone)]
pub struct Rectangle{
    pub x : f64,
    pub y : f64,
    pub w : f64,
    pub h : f64,
}
impl Rectangle{
    pub fn top(&self) -> f64{
        self.y
    }
    pub fn left(&self) -> f64{
        self.x
    }
    pub fn width(&self) -> u32{
        self.w as u32    }
    pub fn height(&self) -> u32{
        self.h as u32
    }
    pub fn right(&self) -> f64{
        self.x +  self.w
    }
    pub fn bottom(&self) -> f64{
        self.y + self.h
    }
    pub fn center(&self) -> Vector2D{
        Vector2D{x: (self.left()+self.right())/2.0 , y: (self.top()+self.bottom())/2.0}
    }
    pub fn center_point(&self) -> Point{
        return Point::new(((self.left()+self.right())/2.0) as i32, ((self.top()+self.bottom())/2.0) as i32);
    }
}
pub struct Rigidbody{ 
    pub hitbox : Rectangle,
    pub vel: Vector2D,
    pub accel: Vector2D,
    pub elasticity: f64,
    pub mass: f64,
    pub i_mass: f64,
    pub radius: f64,
    pub s: bool,
    pub friction: f64,
}
impl Copy for Rigidbody { }
impl Clone for Rigidbody {
    fn clone(&self) -> Rigidbody {
        *self
    }
}
#[allow(dead_code)]

impl Rigidbody{
    pub fn new(rect : Rect, x:f64,y:f64, mass: f64, friction: f64)->Rigidbody{
        let hitbox = Rectangle {x :rect.left() as f64, y: rect.top() as f64, w: rect.width() as f64, h: rect.height() as f64};
        let vel = Vector2D {x, y};
        let accel = Vector2D{x:0.0, y: 0.0};
        let i_mass = 1.0/mass;
        let elasticity  =1.0;
        let radius = hitbox.center().x-hitbox.right();
        let s = false;
        Rigidbody{
            hitbox,
            vel,
            accel,
            elasticity, 
            mass,
            i_mass,
            radius,
            s,
            friction,
        }
    }
    pub fn new_static(rect : Rect, x:f64,y:f64, mass: f64)->Rigidbody{
        let hitbox = Rectangle {x :rect.left() as f64, y: rect.top() as f64, w: rect.width() as f64, h: rect.height() as f64};
        let vel = Vector2D {x, y};
        let accel = Vector2D{x:0.0, y: 0.0};
        let i_mass = 1.0/mass;
        let elasticity  =1.0;
        let radius = TILE_SIZE_PROJECTILE as f64/3.0;
        let s = true;
        let friction = 0.0;
        Rigidbody{
           hitbox,
            vel,
            accel,
            elasticity, 
            mass,
            i_mass,
            radius,
            s,
            friction,

        }
    }
    pub fn pos(self)->Rect{
        Rect::new(self.hitbox.x as i32, self.hitbox.y as i32, self.hitbox.w as u32, self.hitbox.h as u32)
    }
    pub fn change_velocity(&mut self, vel: Vector2D){
        self.vel = vel;
    }
    pub fn change_accel(&mut self, accel: Vector2D){
        self.accel = accel;
    }
    pub fn update_pos(&mut self){
        self.hitbox = Rectangle{x: self.hitbox.x + self.vel.x, y:  self.hitbox.y + self.vel.y, w: self.hitbox.w ,h: self.hitbox.h}
    }
    pub fn check_rect_col(self, other: Rigidbody) -> bool {
        if self.hitbox.bottom() < other.hitbox.top() || self.hitbox.top() > other.hitbox.bottom()|| self.hitbox.right() < other.hitbox.left()|| self.hitbox.left() > other.hitbox.right()
        { false } else { true }
    }
    
    pub fn rect_vs_rect(self, other: Rigidbody, normal_collision : &mut Vector2D, pen: &mut f64)->bool{ // farnan SAT collision detection 
        let vec_from_a_to_b =  Vector2D{x:other.hitbox.x , y: other.hitbox.y} - Vector2D{x:self.hitbox.x , y:self.hitbox.y} ;
        let a = self.hitbox;
        let b = other.hitbox;

        let overlap_x = ((a.right() - a.left()) / 2.0) + ((b.right() - b.left())/2.0) - f64::abs(vec_from_a_to_b.x);

        if overlap_x > 0.0{
            let overlap_y = ((a.bottom() - a.top())/2.0) + ((b.bottom() - b.top())/2.0) - f64::abs(vec_from_a_to_b.y);
            if overlap_y > 0.0{
                if overlap_x < overlap_y{
                    if vec_from_a_to_b.x < 0.0 {
                        *normal_collision = Vector2D{x : -1.0, y : 0.0};
                    } else {
                        *normal_collision = Vector2D{x : 1.0, y : 0.0};
                    }
                    *pen = overlap_x;
                    return true;
                } else {
                    if vec_from_a_to_b.y < 0.0 {
                        *normal_collision = Vector2D{x : 0.0 , y : -1.0};
                    }else{
                        *normal_collision = Vector2D{x : 0.0, y : 1.0};
                    }
                    *pen = overlap_y;
                    return true;
                }
            } else {
                return false
            }
        } else {
            false
        }

    }

    pub fn circle_vs_circle(self, other: Rigidbody, normal_collision : &mut Vector2D, pen: &mut f64)->bool{
        // Vector from A to B
        let r = self.radius + other.radius;//Ra + Rb
        let r_square = r * r;
        let n =  other.hitbox.center() - self.hitbox.center();
                             

        let r = (self.hitbox.right() - self.hitbox.left() / 2.0) + (other.hitbox.right() - other.hitbox.left() / 2.0);

        if n.length_squared() > r_square {
            return false;
        }

        // Circles have collided, compute manifold
        let distance = n.length();

        if distance != 0.0 {
            // Distance is difference between radius and distance
            *pen = r - distance;
            *normal_collision = n.normalize();//distance;
            return true;
        } else {
            // Circles are on same position
            *pen = self.radius;//Ra
            *normal_collision = Vector2D { x: 1.0, y: 0.0 };
            return true;
        }
    }

    pub fn rect_vs_circle(self, other: Rigidbody, normal_collision : &mut Vector2D, pen: &mut f64) -> bool {
        let a_to_b = other.hitbox.center() - self.hitbox.center() ;

        let mut closest_point = a_to_b;
        let self_x_extreme = (self.hitbox.right() - self.hitbox.left()) / 2.0;
        let self_y_extreme = (self.hitbox.bottom() - self.hitbox.top()) / 2.0;

        closest_point.x = closest_point.x.clamp(-self_x_extreme,self_x_extreme);
        closest_point.y = closest_point.y.clamp(-self_y_extreme,self_y_extreme);
        
        let mut inside = false;
        if a_to_b == closest_point{
            inside = true;
            if  f64::abs(a_to_b.x) > f64::abs(a_to_b.y) {
                if closest_point.x > 0.0 {
                    closest_point.x = self_x_extreme;
                } else {
                    closest_point.x = -self_x_extreme;
                }
            }else{
                if closest_point.y > 0.0 {
                    closest_point.y = self_y_extreme;
                } else {
                    closest_point.y = -self_y_extreme;
                }
            }
        }
        let normal = a_to_b - closest_point;
        let mut d = normal.length_squared();
        let r = other.radius;
        
        if d>r*r && !inside{
            return false
        }
        d = d.sqrt();
        if inside{
            *normal_collision = -a_to_b.normalize();
            *pen = r - d;
        }
        else{
            *normal_collision = a_to_b.normalize();
            *pen = r - d;
        }
        true
    }

    pub fn resolve_col(&mut self, other: &mut Rigidbody, normal_collision : Vector2D, _pen: f64){
      
        let normal_vel = (other.vel - self.vel) * (normal_collision);
        if normal_vel > 0.0{
            return;
        } 
        
        let imp_scalar = (-(1.0 + f64::min(self.elasticity,other.elasticity)) * normal_vel) / (self.i_mass + other.i_mass);
        let impulse_vec = normal_collision * imp_scalar;
        if !self.s{
            self.vel = self.vel - ((self.i_mass) * impulse_vec);
        }
        if !other.s{
            other.vel = other.vel + ((other.i_mass) * impulse_vec);
        }
       
    }

    // IN PROGRESS: might use later for very fast objects
    /* fn swept(self, other: Rigidbody,  normal_x : &mut f64,  normal_y : &mut f64 )-> f64{//moving self and other is static
        let inv_entry_x : f64;
        let inv_entry_y : f64;
        let inv_exit_x : f64;
        let inv_exit_y: f64;

        if self.vel.x > 0.0 {
            inv_entry_x = other.hitbox.x - (self.hitbox.x + self.hitbox.w);
            inv_exit_x = (other.hitbox.x+ other.hitbox.w) - self.hitbox.x;
        }else{
            inv_entry_x = (other.hitbox.x + other.hitbox.w) -self.hitbox.x;
            inv_exit_x = other.hitbox.x - (self.hitbox.x + self.hitbox.w);
        }
        if self.vel.y > 0.0 {
            inv_entry_y = other.hitbox.y - (self.hitbox.y + self.hitbox.h);
            inv_exit_y = (other.hitbox.y+ other.hitbox.h) - self.hitbox.y;
        }else{
            inv_entry_y = (other.hitbox.y + other.hitbox.h) -self.hitbox.y;
            inv_exit_y = other.hitbox.y - (self.hitbox.y + self.hitbox.h);
        }
        let entry_x : f64;
        let entry_y : f64;
        let exit_x : f64;
        let exit_y : f64;
    }
   */
}