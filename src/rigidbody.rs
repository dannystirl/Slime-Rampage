extern crate rogue_sdl;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::sys::exit;
use crate::vector::*;

#[derive(Copy,Clone)]
struct Rectangle{
    x : f64,
    y : f64,
    w : f64,
    h : f64,
}
impl Rectangle{
    pub fn top(&self) -> f64{
        self.y
    }
    pub fn left(&self) -> f64{
        self.x
    }
    pub fn right(&self) -> f64{
        self.x +  self.w
    }
    pub fn bottom(&self) -> f64{
        self.y + self.h
    }
}
pub struct Rigidbody{
    
    hitbox : Rectangle,
    vel: Vector2D,
    elasticity: f64,
    mass: f64,
    
}
impl Copy for Rigidbody { }

impl Clone for Rigidbody {
    fn clone(&self) -> Rigidbody {
        *self
    }
}
impl Rigidbody{
    pub fn new(rect : Rect, x:f64,y:f64, mass: f64)->Rigidbody{
        let hitbox = Rectangle {x :rect.left() as f64, y: rect.top() as f64, w: rect.width() as f64, h: rect.height() as f64};
        let vel = Vector2D {x, y};
        let elasticity  =1.0;
        Rigidbody{
            hitbox,
            vel,
            elasticity, 
            mass,
        }
    }
    pub fn draw_pos(self)->Rect{
        Rect::new(self.hitbox.x as i32, self.hitbox.y as i32, self.hitbox.w as u32, self.hitbox.h as u32)
    }
    pub fn change_velocity(&mut self, vel: Vector2D){
        self.vel = vel;
    }
    pub fn update_pos(&mut self){
        self.hitbox = Rectangle{x: self.hitbox.x + self.vel.x, y:  self.hitbox.y + self.vel.y, w: self.hitbox.w ,h: self.hitbox.h}
    }
    
    pub fn check_rect_col(self, other: Rigidbody )->bool{ // farnan SAT collision detection 
        if self.hitbox.bottom() < other.hitbox.top() || self.hitbox.top() > other.hitbox.bottom()|| self.hitbox.right() < other.hitbox.left()|| self.hitbox.left() > other.hitbox.right()
        {
            false
        }else{
            true
        }
    }
    pub fn resolve_col(&mut self, other: &mut Rigidbody){
        
        let normal_vel = (other.vel - self.vel) *(self.vel ).normalize();
        if normal_vel > 0.0{
            println!("nothin");
            return;
        }
        let imp_scalar = (-(1.0 + f64::min(self.elasticity,other.elasticity)) * normal_vel)/(1.0/self.mass +1.0/other.mass);
        let impulse_vec = (self.vel -other.vel).normalize()*imp_scalar;
        self.vel = self.vel - (1.0 / self.mass * impulse_vec);
        other.vel = other.vel + (1.0 / other.mass * impulse_vec);

    }
    //might use later for very fast objects
    fn swept(self, other: Rigidbody,  normal_x : &mut f64,  normal_y : &mut f64 )-> f64{//moving self and other is static
      
      
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
        
        if self.vel.x == 0.0{
            entry_x = -f64::INFINITY;
            exit_x = f64::INFINITY;
        }else{
            entry_x = inv_entry_x/self.vel.x;
            exit_x = inv_exit_x/self.vel.x;
        }
        if self.vel.y == 0.0{
            entry_y = -f64::INFINITY;
            exit_y = f64::INFINITY;
        }else{
            entry_y = inv_entry_y/self.vel.y;
            exit_y = inv_exit_y/self.vel.y;
        }
        let time_of_entry = entry_x.max(entry_y);
        let time_of_exit = exit_x.min(exit_y);

        if time_of_entry > time_of_exit || entry_x < 0.0 && entry_y < 0.0 || entry_x > 1.0 || entry_y > 1.0{
            *normal_x = 0.0;
            *normal_y = 0.0;
            return 1.0
        }else{
            if entry_x > entry_y{
                if inv_entry_x < 0.0{
                   *normal_x = 1.0;
                   *normal_y = 0.0;
                }else{
                    *normal_x = -1.0;
                    *normal_y = 0.0; 
                }
            }
            else {
                if inv_entry_x < 0.0{
                    *normal_x = 0.0;
                    *normal_y = 1.0;
                }else{
                    *normal_x = 0.0;
                    *normal_y = -1.0; 
                }
            }
        }
        time_of_entry
    }
 
}

