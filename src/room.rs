use rand::Rng;

const TILE_SIZE: u32 = 64;

pub struct Room{
    pub num _rooms: i32, 
    pub xwalls: (i32, i32), 
    pub ywalls: (i32, i32), 
    pub xbounds: (i32, i32), 
    pub ybounds: (i32, i32), 
    pub tiles: Vec<(bool,i32)>,
}

impl Room{
    pub fn new() -> Room{
        let xwalls: (i32, i32),  
        let ywalls: (i32, i32), 
        let xbounds: (i32, i32), 
        let ybounds: (i32, i32), 
        let tiles: Vec<(bool,i32)> = vec![(true,0); ((xwalls.1+2)*(ywalls.1+1)) as usize]; // (draw?, texture)
        let num_rooms = create_rooms(&mut xwalls, &mut ywalls, &mut xbounds, &mut ybounds);
        Room{
            num_rooms, 
            xbounds, 
            ybounds, 
            xwalls, 
            ywalls,
            tiles, 
        }
    }
}

pub fn create_rooms(&mut self, num_rooms: i32) -> i32 {
    let num_rooms = 1; // temp for creating one room
    let mut i = 0;
    while i < num_rooms {
        xwalls[i] = (1,rand::thread_rng().gen_range(19..27));
        ywalls[i] = (1,rand::thread_rng().gen_range(10..19));
        xbounds[i] = ((xwalls.0*TILE_SIZE as i32), ( (xwalls.1 as u32 *TILE_SIZE)-TILE_SIZE) as i32);
        ybounds[i] = ((ywalls.0*TILE_SIZE as i32), ( (ywalls.1 as u32 *TILE_SIZE)-TILE_SIZE) as i32);
        i+=1;
        room_obstacles = create_new_map();
    }
    return num_rooms; 
}

pub fn create_new_map(&mut self) -> Vec<(i32,i32)> {
    let mut obs: Vec<(i32,i32)> = vec![(0,0);0];
    let mut n = 0;
    for i in 0..xwalls.1+1 {
        for j in 0..ywalls.1+1 {
            if i==0 || i==xwalls.1 || j==0 || j==ywalls.1 { // border
                self.tiles[n].0 = true;
                self.tiles[n].1 = 6;
            } else if i==xwalls.0 || i==xwalls.1-1 || j==ywalls.0 || j==ywalls.1-1 { // border-1 random tiles
                let num = rand::thread_rng().gen_range(0..5);
                self.tiles[n].0 = true;
                self.tiles[n].1 = num;
            } else { // obstacles / nothing
                let num = rand::thread_rng().gen_range(0..75);
                if num==7 && self.tiles[n].0==true { 
                    obs.push((i,j));
                    self.tiles[n].1 = num;
                    // prevent overlap
                    self.tiles[n].0 = true;
                    self.tiles[n+1].0=false;
                    self.tiles[n+ywalls.1 as usize].0=false;
                    self.tiles[n+ywalls.1 as usize+1].0=false;
                    self.tiles[n+ywalls.1 as usize+2].0=false;

                } else {
                    self.tiles[n].0 = false;
                }
            }
            n+=1;
        }
    }
    return obs;
    
}