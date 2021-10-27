extern crate rogue_sdl;
use rand::Rng;

const TILE_SIZE: u32 = 64;

pub struct Room{
    pub xwalls: (i32, i32), 
    pub ywalls: (i32, i32), 
    pub xbounds: (i32, i32), 
    pub ybounds: (i32, i32), 
    pub tiles: Vec<(bool,i32)>,
    pub obstacles: Vec<(i32,i32)>, 
}

impl Room{
    pub fn new() -> Room{
        let xwalls = (1,rand::thread_rng().gen_range(19..27));
        let ywalls = (1,rand::thread_rng().gen_range(10..19));
        let xbounds = ((xwalls.0*TILE_SIZE as i32), ( (xwalls.1 as u32 *TILE_SIZE)-TILE_SIZE) as i32);
        let ybounds = ((ywalls.0*TILE_SIZE as i32), ( (ywalls.1 as u32 *TILE_SIZE)-TILE_SIZE) as i32);
        let tiles: Vec<(bool,i32)> = vec![(true,0); ((xwalls.1+2)*(ywalls.1+1)) as usize]; // (draw?, texture)
        let obstacles = create_new_map(xwalls, ywalls, tiles);
        Room{
            xbounds, 
            ybounds, 
            xwalls, 
            ywalls,
            tiles, 
            obstacles, 
        }
    }
}

// helper function to create obstacles for a map. requires input values for unintialized rooms
pub fn create_new_map(xwalls: (i32,i32), ywalls: (i32,i32), tiles: Vec<(bool,i32)> ) -> Vec<(i32,i32)> {
    let mut obs: Vec<(i32,i32)> = vec![(0,0);0];
    let mut n = 0;
    for i in 0..xwalls.1+1 {
        for j in 0..ywalls.1+1 {
            if i==0 || i==xwalls.1 || j==0 || j==ywalls.1 { // border
                tiles[n].0 = true;
                tiles[n].1 = 6;
            } else if i==xwalls.0 || i==xwalls.1-1 || j==ywalls.0 || j==ywalls.1-1 { // border-1 random tiles
                let num = rand::thread_rng().gen_range(0..5);
                tiles[n].0 = true;
                tiles[n].1 = num;
            } else { // obstacles / nothing
                let num = rand::thread_rng().gen_range(0..75);
                if num==7 && tiles[n].0==true { 
                    obs.push((i,j));
                    tiles[n].1 = num;
                    // prevent overlap
                    tiles[n].0 = true;
                    tiles[n+1].0=false;
                    tiles[n+ywalls.1 as usize].0=false;
                    tiles[n+ywalls.1 as usize+1].0=false;
                    tiles[n+ywalls.1 as usize+2].0=false;

                } else {
                    tiles[n].0 = false;
                }
            }
            n+=1;
        }
    }
    return obs;   
}

// background should be moved into rooms