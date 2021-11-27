extern crate rogue_sdl;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use crate::vector::*;

struct Rectangle{
    x : f64,
    y : f64,
    w : f64,
    h : f64,
}
pub struct Rigidbody{
    
    hitbox : Rectangle,
    vel: Vector2D,
    
    //mass: f64,
}
impl Rigidbody{
    //work in progress
    fn swept(self, other: Rigidbody)-> f64{//moving self and other is static
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

        0.0
    }
}

