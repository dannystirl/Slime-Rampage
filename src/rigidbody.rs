extern crate rogue_sdl;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use std::ops::Sub;
use std::mem;
use std::cmp;
pub struct Pointf{
    pub x: f64,
    pub y: f64,
}

 impl Pointf{
    pub fn new(x: f64, y: f64) -> Pointf {
        Pointf{
            x,
            y,
        }
    }
}
 

#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub struct Rigidbody{
    pos: Rect,          //world position of the body
    vel: (f64, f64),    //velocity vector
    dynamic: bool,      //can the body move
}

#[allow(dead_code)]
impl Rigidbody{
    pub fn new(pos: Rect, dynamic: bool)->Rigidbody{
        let vel = (0.0,0.0);
        Rigidbody{
            pos,
            vel,
            dynamic,
        }
    }
    
    //we might have to change this later
    pub fn point_vs_rect(&self,p : Point, r : &Rect) -> bool {
        return p.x() >= r.left() && p.y() >= r.top() && p.x() < r.right() && p.y() < r.bottom();
    }

    pub fn ray_vs_rect(&self, origin : Point , dir : Point, other : Rect, mut time : i32)->bool{
       let contact = Point::new(0,0);
       let inverse_x = 1.0/ (dir.x as f64);
       let inverse_y = 1.0/ (dir.y as f64);
       let inverse_dir = Point::new(inverse_x as i32, inverse_y as i32);

       let inter  = self.pos.intersect_line( origin, dir);
       let mut intersection;
       match inter {
           None=> {return false;}
           Some(inter)=> {intersection = inter;}
       }
       let mut near =intersection.0;
       let mut far = intersection.1;
       //sort distances
       if near.x > far.x{mem::swap(&mut near.x,&mut far.x);}
       if near.y > far.y{mem::swap(&mut near.y,&mut far.y);}
       if near.x > far.y || near.y > near.x{return false;}
       
       //closest place on the line that will be the first contact
       time = cmp::max(near.x, near.y);
       let time_far = cmp::min(far.x, far.y);
       if time_far < 0 { return false; }
       let contact = Point::new(origin.x+time * dir.x,origin.y+time * dir.y );

       let mut normal_contact;
       if near.x >near.y {
           if inverse_dir.x <0 {
               normal_contact = Point::new(1,0);
               //add set method here
           }else{
               normal_contact =  Point::new(-1,0);
               //add set method here
           }
        }else if near.x <near.y {
            if inverse_dir.y <0 {
                normal_contact = Point::new(0,1);
                //add set method here
            }else{
                normal_contact =  Point::new(0,-1);
                //add set method here
            }
        }
           
     
       //let mut other_near = other.top_left().sub(origin);
       //other_near.x = other.x * inverse_x as i32;
       //other_near.y= other.y * inverse_y as i32;
       //let mut other_far = other.bottom_right().sub(origin);
       //other_far = other.x * inverse_x as i32;
       //other_far = other.y * inverse_y as i32;

      // if(other_near.x  > other_far.y){
           
       //}
      true
        
    }
     

    // Check for collision of a moving body with static body
    pub fn dynamic_vs_static(&self, target: &Rigidbody) -> bool{

        // TODO: Check static vs. dynamic

        return false;
    }

    // Check for collision of a moving body with dynamic body
    pub fn dynamic_vs_dynamic(&self, target: &Rigidbody) -> bool{

        //TODO: Check dynamic vs. dynamic

        return false;
    }

    pub fn rect_vs_rect(&self, other :&Rect)->bool{// Stolen from farnans code
        
            if self.pos.bottom() < other.top()
                || self.pos.top() > other.bottom()
                || self.pos.right() < other.left()
                || self.pos.left() > other.right()
            {
                false
            }
            else {
                true
            }
    }

    pub fn pos(&self) -> Rect{
        return self.pos;
    }
    pub fn vel(&self) -> (f64, f64){
        return self.vel;
    }
    pub fn set_pos(&mut self, pos: Rect){
        self.pos = pos;
    }
    pub fn set_vel(&mut self, vel: (f64, f64)){
        self.vel = vel;
    }
    pub fn dynamic(&self) -> bool{
        return self.dynamic;
    }

}