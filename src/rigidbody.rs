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
    vel: Point,    //velocity vector
    dynamic: bool,      //can the body move
    normal_contact: Point, //last contact point
}

#[allow(dead_code)]
impl Rigidbody{
    pub fn new(pos: Rect, dynamic: bool)->Rigidbody{
        let vel = Point::new(0, 0);
        let normal_contact = Point::new(0, 0);
        Rigidbody{
            pos,
            vel,
            dynamic,
            normal_contact,
        }
    }
    
    //we might have to change this later
    pub fn point_vs_rect(&self,p : Point, r : &Rect) -> bool {
        return p.x() >= r.left() && p.y() >= r.top() && p.x() < r.right() && p.y() < r.bottom();
    }

    pub fn ray_vs_rect(&mut self, origin : Point , dir : Point, other : Rect, mut time : i32)->bool{
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

       if near.x >near.y {
           if inverse_dir.x <0 {
               self.normal_contact = Point::new(1,0);
               //add set method here
           }else{
               self.normal_contact =  Point::new(-1,0);
               //add set method here
           }
        }else if near.x <near.y {
            if inverse_dir.y <0 {
                self.normal_contact = Point::new(0,1);
                //add set method here
            }else{
                self.normal_contact =  Point::new(0,-1);
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
        //expanded_target.pos = r_static.pos - r_dynamic->size / 2;
		//expanded_target.size = r_static.size + r_dynamic->size;


        return false;
    }

    // Check for collision of a moving body with dynamic body
    pub fn dynamic_vs_dynamic(&self, target: &Rigidbody) -> bool{

        //TODO: Check dynamic vs. dynamic

        return false;
    }

    pub fn rect_vs_rect(&mut self, other: &Rigidbody, time: i32)-> bool{//time should be float?
            /*
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
            */
            //expand target collision by player dimensions
            if self.vel.x == 0 && self.vel.y == 0 {
                return false;
            }
            let expanded_pos = Rect::new(other.pos.x - self.pos.x / 2 as i32, other.pos.y + self.pos.y/2 as i32, (64 + self.pos.x) as u32, (64 + self.pos.y) as u32);
            if self.ray_vs_rect(other.pos.center(), other.vel, expanded_pos, time) {
                return (time >= 0 && time < 1);
            }
            else {
                return false;
            }
                
    }

    pub fn resolve_dynamic_rects(&mut self, other: &Rigidbody, time: i32) -> bool {
        let time = 0;

        if self.rect_vs_rect(other, time) {
            if self.normal_contact.y > 0 {
                
            }
            if self.normal_contact.x < 0 {

            }
            if self.normal_contact.y < 0 {

            }
            if self.normal_contact.x > 0 {

            }
           
            return true;
        }
        return false; 
    }

    pub fn pos(&self) -> Rect{
        return self.pos;
    }
    pub fn vel(&self) -> Point{
        return self.vel;
    }
    pub fn set_pos(&mut self, pos: Rect){
        self.pos = pos;
    }
    pub fn set_vel(&mut self, vel: Point){
        self.vel = vel;
    }
    pub fn dynamic(&self) -> bool{
        return self.dynamic;
    }

}