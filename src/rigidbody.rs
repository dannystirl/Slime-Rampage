extern crate rogue_sdl;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use std::ops::Sub;
use std::mem;
use std::cmp;
use std::f64;
 

#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub struct Rigidbody{
    pos: (f64, f64),          //world position of the body
    vel: (f64,f64),    //velocity vector
    size: i32,
    dynamic: bool,      //can the body move
    normal_contact: (f64,f64), //last contact point
}

#[allow(dead_code)]
impl Rigidbody{
    pub fn new(pos: (f64,f64), dynamic: bool)->Rigidbody{
        let vel = (0.0,0.0);
        let normal_contact = (0.0,0.0);
        let size = 64;
        Rigidbody{
            pos,
            vel,
            size,
            dynamic,
            normal_contact,
        }
    }
    
    //we might have to change this later
    pub fn point_vs_rect(&self,p : Point, r : &Rect) -> bool {
        return p.x() >= r.left() && p.y() >= r.top() && p.x() < r.right() && p.y() < r.bottom();
    }

    pub fn ray_vs_rect(&mut self, origin : (f64,f64) , dir : (f64,f64), other : Rect, mut time_near : f64)->bool{
       let contact = Point::new(0,0);
       let inverse_x = 1.0/ (dir.0);
       let inverse_y = 1.0/ (dir.1);
       let origin_p = Point::new(origin.0 as i32, origin.1 as i32);
       let dir_p = Point::new(dir.0 as i32, dir.1 as i32);

       let inverse_dir = Point::new(inverse_x as i32, inverse_y as i32);

       let inter  = self.pos().intersect_line(origin_p, dir_p);
       let mut intersection;
       match inter {
           None=> {return false;}
           Some(inter)=> {intersection = inter;}
       }
       let mut near = intersection.0;
       let mut far = intersection.1;
       //sort distances
       if near.x > far.x { mem::swap(&mut near.x, &mut far.x); }
       if near.y > far.y { mem::swap(&mut near.y, &mut far.y); }
       if near.x > far.y || near.y > near.x { return false; }
       
       //closest place on the line that will be the first contact
       if near.x >= near.y{
           time_near =  near.x as f64;
       }else{
        time_near =  near.x as f64;
       }

       let mut time_far = far.y as f64;

       if far.x >= far.y {time_far = far.x as f64;  }


       if time_far < 0.0 { return false; }
       let contact = Point::new((origin.0 +time_near * dir.0)as i32,(origin.1+time_near * dir.1)as i32 );

       if near.x >near.y {
           if inverse_dir.x <0 {
               self.normal_contact = (1.0,0.0);//Point::new(1,0);
               //add set method here
           }else{
               self.normal_contact =  (-1.0,0.0);//Point::new(-1,0);
               //add set method here
           }
        }else if near.x <near.y {
            if inverse_dir.y <0 {
                self.normal_contact = (0.0,1.0);//Point::new(0,1);
                //add set method here
            }else{
                self.normal_contact = (0.0,-1.0);//Point::new(0,-1);
                //add set method here
            }
        }
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

    pub fn rect_vs_rect(&mut self, other: &Rigidbody, contact_time: f64)-> bool{
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
            if self.vel.0 == 0.0 && self.vel.1 == 0.0 {
                return false;
            }
            let expanded_pos = Rect::new((other.pos.0 - self.pos.0 / 2.0) as i32, (other.pos.0 + self.pos.1 / 2.0) as i32, (self.size as f64+ self.pos.0) as u32, (self.size as f64 + self.pos.1) as u32);
            if self.ray_vs_rect(other.pos, other.vel, expanded_pos, contact_time as f64) {
                return (contact_time >= 0.0 && contact_time < 1.0);
            }
            else {
                return false;
            }
                
    }

    pub fn resolve_dynamic_rects(&mut self, other: &Rigidbody, time: i32) -> bool {
        let time = 0.0;
        //
        if self.rect_vs_rect(other, time) {
            if self.normal_contact.1 > 0.0 {
                
            }
            if self.normal_contact.0 < 0.0 {

            }
            if self.normal_contact.1 < 0.0 {

            }
            if self.normal_contact.0 > 0.0 {

            }

            //r_dynamic->vel += contact_normal * olc::vf2d(std::abs(r_dynamic->vel.x), std::abs(r_dynamic->vel.y)) * (1 - contact_time);
            //scalar: (1 - contact_time); contact_normal indicates the direction that vel should be (I believed)
            let v = (self.vel.0, self.vel.1);
            return true;
        }
        return false; 
    }

    
    
    
    pub fn x(&self) -> f64 {
		return self.pos.0;
	}
    
    pub fn y(&self) -> f64 {
		return self.pos.1;
	}

    pub fn pos(&self) -> Rect {
        return Rect::new(
            self.x() as i32,
            self.y() as i32,
            self.size() as u32,
            self.size() as u32,
        )  
    }
    pub fn size(&self) -> i32 {
		return self.size;
	}
    pub fn vel(&self) -> (f64,f64){
        return self.vel;
    }
    pub fn set_pos(&mut self, pos: (f64,f64)){
        self.pos = pos;
    }
    pub fn set_vel(&mut self, vel: (f64, f64)){
        self.vel = vel;
    }
    pub fn dynamic(&self) -> bool{
        return self.dynamic;
    }
}