extern crate rogue_sdl;
use sdl2::rect::Rect;
use sdl2::rect::Point;

/*
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
 */

#[derive(Copy, Clone)]
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

    /*
    pub fn ray_vs_rect(&self, origin : Pointf , dir : Pointf, other : Rect, hit_near : f64){
       let contact = Pointf::new(0.0,0.0);
       let normal = Pointf::new(0.0,0.0);
       let inverse_x = 1.0/ (dir.x as f64);
       let inverse_y = 1.0/ (dir.y as f64);
       let inverse_dir = Pointf::new(inverse_x, inverse_y);
        
    }
     */

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