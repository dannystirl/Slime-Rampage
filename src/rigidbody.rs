extern crate rogue_sdl;
use sdl2::rect::Rect;

pub struct Rigidbody{
    pos: Rect,
    vel: (f64, f64),
    //mass: f64,
}

impl Rigidbody{

    pub fn new(pos: Rect)->Rigidbody{
        let vel = (0.0,0.0);
        Rigidbody{
            pos,
            vel,
        }
    }
    
//we might have to change this later
    pub fn rect_vs_rect(&self, other :&Rect)->bool{// thanks Farnan
        
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

}