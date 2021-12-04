extern crate rogue_sdl;
use std::cmp;
use rand::Rng;

use crate::gamedata::*;
use crate::background::*;

pub struct Map<'a> {
	pub background: Background<'a>, 
	pub current_floor: i32, 
	pub map: [[i32; MAP_SIZE_W]; MAP_SIZE_H],
	pub numbered_map: [[i32; MAP_SIZE_W]; MAP_SIZE_H],
	pub room_sizes: Vec<(usize,usize,usize,usize)>,
	pub num_rooms: i32, 
	pub starting_room: i32, 
	pub starting_position: (f64,f64), 
	pub ending_room: i32, 
	pub ending_position: (i32,i32), 
	pub shop: i32,
	pub enemy_and_object_spawns: [[i32; MAP_SIZE_W]; MAP_SIZE_H],
	pub shop_spawns: Vec<(usize, usize)>, 
	pub shop_items: Vec<(ShopItems, bool, u32)>, // item type, bought, cost
	pub shop_creation: Vec<i32>, 
}

pub enum ShopItems{
	Fireball, 
	Slimeball,
	Rock,
	Shield,
	Dash,
	HealthUpgrade, 
	Health,
	Sword,
	Spear,
	None,
}

impl<'a> Map<'a> {
	pub fn new(current_floor: i32, background: Background<'a>) -> Map<'a> { 
		let map = [[0; MAP_SIZE_W]; MAP_SIZE_H]; 
		let numbered_map = [[0; MAP_SIZE_W]; MAP_SIZE_H]; 
		let room_sizes = Vec::with_capacity(0);
		let num_rooms=1; 
		let starting_room = 1;
		let starting_position = (0.0,0.0);
		let ending_room = 2;
		let ending_position = (0,0);
		let shop = 3;
		let enemy_and_object_spawns = [[0; MAP_SIZE_W]; MAP_SIZE_H]; 
		let shop_spawns = Vec::with_capacity(0);
		let shop_items = Vec::with_capacity(0);
		let shop_creation = Vec::with_capacity(0);

		Map {
			background, 
			current_floor, 
			map, 
			numbered_map, 
			room_sizes, 
			num_rooms, 
			starting_room, 
			starting_position, 
			ending_room, 
			ending_position, 
			shop,
			enemy_and_object_spawns, 
			shop_spawns, 
			shop_items, 
			shop_creation, 
		}
	}

	pub fn create_boss(&mut self) {
		let mut rng = rand::thread_rng();
		self.map = [[0; MAP_SIZE_W]; MAP_SIZE_H];

		let mut new_map = self.map;
		for h in 0..BOSS_ROOM_H {
			for w in 0..BOSS_ROOM_W {
				if h == 0 || h == BOSS_ROOM_H - 1 || w == 0 || w == BOSS_ROOM_W - 1 {
					new_map[h][w] = 2;
				} else {
					new_map[h][w] = 1;
				}
			}
		}

		for h in 0..BOSS_ROOM_H {
			for w in 0..BOSS_ROOM_W {
				if (h > 0 && h < 6) || (h < BOSS_ROOM_H - 1 && h > BOSS_ROOM_H - 7) ||
					(w > 0 && w < 6) || (w < BOSS_ROOM_W - 1 && w > BOSS_ROOM_W - 7) {
					let pillar = rng.gen_range(0..8);
					if pillar == 0 {
						new_map[h][w] = 2;
					}
				}
			}
		}

		self.map = new_map;

		self.starting_position = (BOSS_ROOM_W as f64 / 2.0, BOSS_ROOM_H as f64 - 7.0);

		self.enemy_and_object_spawns[6][BOSS_ROOM_W / 2] = 6;

		self.print_map(self.map);
	}

	// 1: create a new map
	pub fn create_map(&mut self) {
		self.map = [[0; MAP_SIZE_W]; MAP_SIZE_H];
		if DEVELOP {
			return;
		}

		// create rooms
		self.create_rooms();
		self.numbered_map = self.map;
		// create maze
		let corridors = self.create_maze(); 
		// form levels
		self.connect_maze();
		self.remove_dead_ends();
		self.create_walls();
		// add objects and entities
		self.create_obstacles(corridors);
		self.create_enemies();
		self.create_objects();
		
		if DEBUG { self.print_map(self.map); }
	}

	// 2: create randomized rooms
	pub fn create_rooms(&mut self) {
		let mut rng = rand::thread_rng();
		let mut new_map = self.map;

		let mut count: i32 = 0;
		while count < 300 {
			let y = rng.gen_range(0..MAP_SIZE_H);
			let x = rng.gen_range(0..MAP_SIZE_W);
			let height = rng.gen_range(MIN_ROOM_H..MAX_ROOM_H);
			let width = rng.gen_range(MIN_ROOM_W..MAX_ROOM_W);
			if y % 2 == 0 || x % 2 == 0 || height % 2 == 0 || width % 2 == 0 {
				continue;
			}
			if y + height < MAP_SIZE_H && x + width < MAP_SIZE_W {
				let mut collided = false;
				for h in 0..height {
					for w in 0..width {
						if x > 2 && y > 2 {
							if new_map[y-1][x-1] != 0 {
								collided = true;
							}
						}
						if y > 2 {
							if new_map[y-1][x+w] != 0 {
								collided = true;
							}
						}
						if x > 2 {
							if new_map[y+h][x-1] != 0 {
								collided = true;
							}
						}
						if new_map[y+h+1][x+w+1] != 0 {
							collided = true;
						}
					}
				}
				if !collided {
					for h in 0..height {
						for w in 0..width {
							new_map[y + h][x + w] = self.num_rooms;
						}
					}
					self.room_sizes.push((y, x, height, width)); 
					self.num_rooms += 1;
				}
				count += 1;
			}
		}
		self.num_rooms -= 1;
		self.map = new_map;
	}

	// 3: create the flood fill maze
	pub fn create_maze(&mut self) -> [[i32; MAP_SIZE_W]; MAP_SIZE_H] {
		let mut recurse: Vec<(usize, usize, (bool,bool,bool,bool), i32)> = Vec::new(); // y, x, direction
		let mut new_map = self.map;
		let mut num_mazes = self.num_rooms;
		for h in (1..MAP_SIZE_H).step_by(2) {
			for w in (1..MAP_SIZE_W).step_by(2) {
				if new_map[h][w] == 0 {
					let y = h;
					let x = w;
					recurse.push((y,x,(false,false,false,false), 4));
					recurse.push((y,x,(false,false,false,false), 4)); // dupe prevents edge case
					num_mazes += 1;
					new_map = self.build_maze(new_map, num_mazes, &mut recurse);
				}
			}
		}

		let mut corridors = self.map;
		for h in 0..MAP_SIZE_H {
			for w in 0..MAP_SIZE_W {
				if new_map[h][w] > self.num_rooms {
					corridors[h][w] = 1;
				} else {
					corridors[h][w] = 0;
				}
			}
		}
		self.map = new_map; 
		return corridors;
	}

	// 3.1: choose a direction for the maze
	pub fn build_maze(&mut self, mut new_map: [[i32; MAP_SIZE_W]; MAP_SIZE_H], num_maze: i32, recurse: &mut Vec<(usize,usize,(bool,bool,bool,bool),i32)>) -> [[i32; MAP_SIZE_W]; MAP_SIZE_H] {
		let mut rec_length = recurse.len()-1;
		let mut y = recurse[rec_length].0;
		let mut x = recurse[rec_length].1;
		new_map[y][x] = num_maze;
		
		while rec_length >= 1 {
			let mut update = false;

			let roll = rand::thread_rng().gen_range(1..4);
			let mut roll_count = 0;
			for direction in 0..4 {
				match direction {
					// NORTH
					0 => {
						if recurse[rec_length].2.0 == false {	// test if already moved west
							roll_count += 1;
							if roll_count == roll {				// random roll check
								recurse[rec_length] = (y,x,(	// update current position's directions
									true,
									recurse[rec_length].2.1,
									recurse[rec_length].2.2,
									recurse[rec_length].2.3), 
									recurse[rec_length].3 - 1);
								if y > 2 && new_map[y-2][x] == 0 { 	// can move direction
									//println!("North");
									recurse.push((y-2,x,(false,false,true,false), 3));	// push a new point for recursion
									rec_length+=1;
									update = true;
									new_map[y-1][x] = num_maze;
									y = y - 2;
								}
							}
						}
					},
					// EAST
					1 => {
						if recurse[rec_length].2.1 == false {
							roll_count += 1;
							if roll_count == roll {
								recurse[rec_length] = (y,x,(
									recurse[rec_length].2.0,
									true,
									recurse[rec_length].2.2,
									recurse[rec_length].2.3), 
									recurse[rec_length].3 - 1);
								if x < MAP_SIZE_W - 2 && new_map[y][x+2] == 0 {
									//println!("East");
									recurse.push((y,x+2,(false,false,false,true), 3));
									rec_length+=1;
									update = true;
									new_map[y][x+1] = num_maze;
									x = x + 2;
								}
							}
						}
					},
					// SOUTH
					2 => {
						if recurse[rec_length].2.2 == false {
							roll_count += 1;
							if roll_count == roll {
								recurse[rec_length] = (y,x,(
									recurse[rec_length].2.0,
									recurse[rec_length].2.1,
									true,
									recurse[rec_length].2.3), 
									recurse[rec_length].3 - 1);
								if y < MAP_SIZE_H - 2 && new_map[y+2][x] == 0 {
									//println!("South");
									recurse.push((y+2,x,(true,false,false,false), 3));
									rec_length+=1;
									update = true;
									new_map[y+1][x] = num_maze;
									y = y + 2;
								}
							}
						}
					},
					// WEST
					_ => {
						if recurse[rec_length].2.3 == false {
							roll_count += 1;
							if roll_count == roll {
								recurse[rec_length] = (y,x,(
									recurse[rec_length].2.0,
									recurse[rec_length].2.1,
									recurse[rec_length].2.2,
									true), 
									recurse[rec_length].3 - 1);
								if x > 2 && new_map[y][x-2] == 0{
									//println!("West");
									recurse.push((y,x-2,(false,true,false,false), 3));
									rec_length+=1;
									update = true;
									new_map[y][x-1] = num_maze;
									x = x - 2;
								}
							}
						}
					},
				}
			}
			if update {
				new_map[y][x] = num_maze;
			} else if recurse[rec_length].3 == 0 {
				recurse.pop();
				rec_length -= 1;
				y = recurse[rec_length].0;
		 		x = recurse[rec_length].1;
			}
		}
		return new_map;
	}

	// 4: connect the finished maze to the rooms
	pub fn connect_maze(&mut self) {
		let mut connectors = self.get_connectors(self.map);
		let mut new_map = self.map;

		// create first door per room
		while connectors.len() > 0 {
			let rand_connection = rand::thread_rng().gen_range(0..connectors.len());
			new_map[connectors[rand_connection].0][connectors[rand_connection].1] = 1;
			// roll for second & third doors
			if rand::thread_rng().gen_range(0..30) < 15 {
				let rand_addition: usize; 
				// attempt to make second door far from the first
				if rand_connection > connectors.len()/2 {
					rand_addition = rand::thread_rng().gen_range(0..connectors.len()/2);
				} else {
					rand_addition = rand::thread_rng().gen_range(connectors.len()/2..connectors.len());
				}
				new_map[connectors[rand_addition].0][connectors[rand_addition].1] = 1;
				if rand::thread_rng().gen_range(0..30) < 5 {
					let rand_addition = rand::thread_rng().gen_range(0..connectors.len());
					new_map[connectors[rand_addition].0][connectors[rand_addition].1] = 1;
				}
			}
			new_map = self.coalesce(connectors[rand_connection].2, connectors[rand_connection].3, new_map);
			connectors = self.get_connectors(new_map);
		}
		self.map = new_map;
	}

	// 4.1: get connectors for the maze and rooms
	pub fn get_connectors(&mut self, map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]) -> Vec<(usize, usize, i32, i32)> {
		let mut connectors: Vec<(usize, usize, i32, i32)> = Vec::new();

		for h in 0..MAP_SIZE_H as i32 {
			for w in 0..MAP_SIZE_W as i32 {
				if map[h as usize][w as usize] != 0 {
					for k in 0..3 as i32 {
						for l in 0..3 as i32 {
							if h + 2 * k - 2 < 0 ||
							   w + 2 * l - 2 < 0 ||
							   h + 2 * k - 2 >= MAP_SIZE_H as i32 ||
							   w + 2 * l - 2 >= MAP_SIZE_W as i32 {
								   continue;
							}
							if map[h as usize + k as usize - 1][w as usize] == 0 && 
							   map[h as usize + 2 * (k as usize) - 2][w as usize] != 0 {
								let r1 = map[h as usize + 2 * (k as usize) - 2][w as usize];
								let r2 = map[h as usize][w as usize];
								if r1 != r2 {
									connectors.push((h as usize + k as usize - 1, w as usize, r1, r2));
								}
							}
							else if map[h as usize][w as usize + l as usize - 1] == 0 && 
									map[h as usize][w as usize + 2 * (l as usize) - 2] != 0 {
								let r1 = map[h as usize][w as usize + 2 * (l as usize) - 2];
								let r2 = map[h as usize][w as usize];
								if r1 != r2 {
									connectors.push((h as usize, w as usize + l as usize - 1, r1, r2));
								}
							}
						}
					}
				}
			}
		}	
		return connectors;
	}

	// 4.2: join rooms and maze corridors/other rooms
	pub fn coalesce(&mut self, r1: i32, r2: i32, map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]) -> [[i32; MAP_SIZE_W]; MAP_SIZE_H] {
		let mut new_map = map;		
		for h in 0..MAP_SIZE_H {
			for w in 0..MAP_SIZE_W {
				if new_map[h][w] == r1 || new_map[h][w] == r2 {
					new_map[h][w] = cmp::min(r1, r2);
					continue; 
				}
			}
		}
		return new_map;
	}

	// 5: remove any maze dead ends
	pub fn remove_dead_ends(&mut self) {
		let mut new_map = self.map;
		let mut still_removing = true;
		
		while still_removing {
			still_removing = false;
			for h in 0..MAP_SIZE_H {
				for w in 0..MAP_SIZE_W {
					if new_map[h][w] == 1 {
						let mut count = 0;
						if new_map[h + 1][w] == 0 {
							count += 1;
						}
						if new_map[h - 1][w] == 0 {
							count += 1;
						}
						if new_map[h][w + 1] == 0 {
							count += 1;
						}
						if new_map[h][w - 1] == 0 {
							count += 1;
						}
						if count >= 3 {
							still_removing = true;
							new_map[h][w] = 0;
						}
					}
				}
			}
		}
		self.map = new_map;
	}

	// 6: create room and corridor walls
	pub fn create_walls(&mut self) {
		let mut new_map = self.map;

		for h in 0..MAP_SIZE_H as i32 {
			for w in 0..MAP_SIZE_W as i32 {
				if new_map[h as usize][w as usize] == 0 {
					for k in 0..3 as i32 {
						for l in 0..3 as i32 {
							if h + k - 1 < 0 ||
							   w + l - 1 < 0 ||
							   h + k >= MAP_SIZE_H as i32 ||
							   w + l >= MAP_SIZE_W as i32 {
								   continue;
							}
							if new_map[h as usize + k as usize - 1][w as usize + l as usize - 1] == 1 {
								//add wall
								let moss = rand::thread_rng().gen_range(0..60);
								if moss < 10 * self.current_floor {
									new_map[h as usize][w as usize] = 5;		// moss walls
								} else { new_map[h as usize][w as usize] = 2; }	// walls
							}
						}
					}
				}
			}
		}
		self.map = new_map;
	}

	// 7: create obstacles, stairs, and other random spawns
	pub fn create_obstacles(&mut self, corridors: [[i32; MAP_SIZE_W]; MAP_SIZE_H]) {
		let mut rng = rand::thread_rng();
		let mut new_map = self.map;
		let mut special_rooms: usize = 0;
		self.room_sizes.sort_by(|a, b| (a.2*a.3).cmp(&(b.2*b.3)) ); 

		// place stairs
		while special_rooms < 2 {
			// set stairs in smaller rooms
			let h = rng.gen_range(self.room_sizes[special_rooms].0..
								  self.room_sizes[special_rooms].0+self.room_sizes[special_rooms].2-1);	
			let w = rng.gen_range(self.room_sizes[special_rooms].1..
								  self.room_sizes[special_rooms].1+self.room_sizes[special_rooms].3-1);
			if new_map[h][w] == 1 && corridors[h][w] != 1 && 
			   self.surroundings_clear(w, h, vec![2], new_map) {

				// Add upstairs (3)
				if special_rooms == 0 {			
					new_map[h][w] = 3;
					self.starting_position = (w as f64, h as f64);
					self.starting_room = self.numbered_map[h][w];
					special_rooms += 1;
				}
				// Add downstairs (4)
				else if special_rooms == 1 {
					if self.num_rooms > 1 && self.numbered_map[h][w] == self.starting_room {
						continue; 
					}
					else {
						new_map[h][w] = 4;
						self.ending_position = (w as i32, h as i32);
						self.ending_room = self.numbered_map[h][w];
						special_rooms += 1;
					}
				}
			}
		}
		// place shop
		while special_rooms < 3 {
			let h = rng.gen_range(self.room_sizes[special_rooms].0..
								  self.room_sizes[special_rooms].0+self.room_sizes[special_rooms].2-1);	
			let w = rng.gen_range(self.room_sizes[special_rooms].1..
								  self.room_sizes[special_rooms].1+self.room_sizes[special_rooms].3-1);
			if new_map[h][w] == 1 && corridors[h][w] != 1 && self.surroundings_clear(w, h, vec![2], new_map) {
			
				if self.num_rooms > 1 && (self.numbered_map[h][w] == self.starting_room || self.numbered_map[h][w] == self.ending_room){
					continue; 
				}
				else {
					if DEBUG { self.starting_position = (w as f64, h as f64); }
					self.shop = self.numbered_map[h][w];
					while self.shop_spawns.len() < 4 {
						let h = rng.gen_range(self.room_sizes[special_rooms].0..
											  self.room_sizes[special_rooms].0+self.room_sizes[special_rooms].2-1);	
						let w = rng.gen_range(self.room_sizes[special_rooms].1..
											  self.room_sizes[special_rooms].1+self.room_sizes[special_rooms].3-1);
						if corridors[h][w] != 1 && self.surroundings_clear(w, h, vec![2, 6], new_map) {
							if !self.shop_spawns.contains(&(h,w)) {
								new_map[h][w] = 6;
								self.shop_spawns.push((h,w)); 
								// should ensure no duplicate powers at some point
								let mut item = rng.gen_range(1..9);
								while self.shop_creation.contains(&item) {
									item = rng.gen_range(1..20);
								}
								// type, purchased, cost
								match item {
									1..=3 => {
										self.shop_items.push((ShopItems::Fireball, false, 3)); 
										self.shop_creation.extend(1..=3); 
									}
									3..=4 => {
                                        self.shop_items.push((ShopItems::Rock, false, 4));
                                        self.shop_creation.extend(3..=4);
                                    }
									4..=5 => {
										self.shop_items.push((ShopItems::Slimeball, false, 2)); 
										self.shop_creation.extend(4..=5); 
									}
									6 => {
										self.shop_items.push((ShopItems::Shield, false, 5)); 
										self.shop_creation.push(6); 
									}
									7 => {
										self.shop_items.push((ShopItems::Dash, false, 4));
										self.shop_creation.push(7); 
									}
									8 => {
										self.shop_items.push((ShopItems::HealthUpgrade, false, 5)); 
										self.shop_creation.push(8); 
									}
									9..=10 => {
										self.shop_items.push((ShopItems::Sword, false, 3));
										self.shop_creation.extend(9..=10); 
									}
									11..=12 => {
										self.shop_items.push((ShopItems::Spear, false, 5));
										self.shop_creation.extend(11..=12); 
									}
									_ => {
										self.shop_items.push((ShopItems::Health, false, 3)); 
										self.shop_creation.push(item); 
									}
								}
							}
						}
					}
					special_rooms += 1;
				}
			}
		}

		//add pillars
		for _i in 1..75 {
			let h = rng.gen_range(0..MAP_SIZE_H - 1);
			let w = rng.gen_range(0..MAP_SIZE_W - 1);
			if new_map[h][w] == 1 && corridors[h][w] != 1 && 
			   self.surroundings_clear(w, h, vec![2], new_map) &&
			   self.numbered_map[h][w] != self.starting_room &&
			   self.numbered_map[h][w] != self.ending_room &&
			   self.numbered_map[h][w] != self.shop {
				//add wall
				let moss = rng.gen_range(0..10);
				if moss < 8 {
					new_map[h][w] = 2;			// pillars
				} else { new_map[h][w] = 5; }	// moss pillars
			}
		}

		self.map = new_map;
	}

	// 8: create enemies
	pub fn create_enemies(&mut self) {
		let mut rng = rand::thread_rng();
		let mut enemy_and_object_spawns = [[0; MAP_SIZE_W]; MAP_SIZE_H];
		let mut spawn_positions: Vec<(usize, usize)>;

		for i in 1..(self.num_rooms + 1) {
			if i == self.starting_room || i == self.ending_room || i == self.shop {
				continue;
			}
			spawn_positions = Vec::new();
			for h in 0..MAP_SIZE_H {
				for w in 0..MAP_SIZE_W {
					if self.numbered_map[h][w] == i {
						spawn_positions.push((h, w));
					}
				}
			}

			let mut tests = 0; 
			let mut enemy_number = vec![0,0,0,0,0,0];
			let enemy_number_max = vec![spawn_positions.len()/32 + rng.gen_range(1..4), 	// total enemies
										rng.gen_range(2..6), 	// ghosts
										rng.gen_range(0..3), 	// gellems
										rng.gen_range(1..3),    // skeletons
										rng.gen_range(1..5),    // eyeballs
										rng.gen_range(1..3)]; 	// rock
			while enemy_number[0] < enemy_number_max[0] && tests < 30 {
				tests += 1; 
				let pos = spawn_positions[rng.gen_range(0..spawn_positions.len())];
				if enemy_and_object_spawns[pos.0][pos.1] != 0 {
					continue;
				}
				
				let enemy = rng.gen_range(1..19);
				match enemy {
					1..=3 => { // gellems
						if enemy_number[2] == enemy_number_max[2] {
							continue; 
						}
						enemy_number[2] += 1; 
						enemy_and_object_spawns[pos.0][pos.1] = 2;
					}
					4..=6 => { // skeletons
						if enemy_number[3] == enemy_number_max[3] {
							continue; 
						}
						enemy_number[3] += 1; 
						enemy_and_object_spawns[pos.0][pos.1] = 4;
					}
					7..=10 => { // eyeballs
						if enemy_number[4] == enemy_number_max[4] {
							continue; 
						}
						enemy_number[4] += 1; 
						enemy_and_object_spawns[pos.0][pos.1] = 5;
					}
					11..=13 => { // rock
                        if enemy_number[5] == enemy_number_max[5] || self.current_floor < 2{
                            continue;
                        }
                        enemy_number[5] += 1;
                        enemy_and_object_spawns[pos.0][pos.1] = 6;
                    }
					_ => { // ghosts
						if enemy_number[1] == enemy_number_max[1] {
							continue; 
						}
						enemy_number[1] += 1; 
						enemy_and_object_spawns[pos.0][pos.1] = 1;
					}
				}
				enemy_number[0] += 1; 
			}
		}
		self.enemy_and_object_spawns = enemy_and_object_spawns;
	}

	// create objects
	pub fn create_objects(&mut self) {
		let mut rng = rand::thread_rng();
		let mut enemy_and_object_spawns = self.enemy_and_object_spawns;
		let mut spawn_positions: Vec<(usize, usize)>;

		for i in 1..(self.num_rooms + 1) {
			if i == self.starting_room || i == self.ending_room || i == self.shop {
				continue;
			}
			spawn_positions = Vec::new();
			for h in 0..MAP_SIZE_H {
				for w in 0..MAP_SIZE_W {
					if self.numbered_map[h][w] == i {
						spawn_positions.push((h, w));
					}
				}
			}

			let crates = rng.gen_range(1..4);
			let mut crates_placed = 0;
			while crates_placed < crates {
				let pos = spawn_positions[rng.gen_range(0..spawn_positions.len())];
				if enemy_and_object_spawns[pos.0][pos.1] != 0 && self.map[pos.0][pos.1] != 1 {
					continue;
				}
				enemy_and_object_spawns[pos.0][pos.1] = 3;
				crates_placed += 1;
			}
		}
		self.enemy_and_object_spawns = enemy_and_object_spawns;
	}

	// check blocks around (w,h) for tile number
	pub fn surroundings_clear(&self, w: usize, h: usize, tiles: Vec<i32>, new_map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]) -> bool {
		for tile in tiles {
			if new_map[h - 1][w] == tile || new_map[h + 1][w] == tile || 
			   new_map[h][w - 1] == tile || new_map[h][w + 1] == tile ||
			   new_map[h - 1][w - 1] == tile || new_map[h - 1][w + 1] == tile ||
			   new_map[h + 1][w - 1] == tile || new_map[h + 1][w + 1] == tile {
			   return false; 
			}
		}
		return true; 
	}

	// print the current map
	pub fn print_map(&self, map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]){
		println!("");
		for h in 0..MAP_SIZE_H {
			for w in 0..MAP_SIZE_W {
				// Ghosts
				if self.enemy_and_object_spawns[h][w] == 1 {
					print!("G ");
				}
				// Gellems
				else if self.enemy_and_object_spawns[h][w] == 2 {
					print!("E ");
				}
				// Crates
				else if self.enemy_and_object_spawns[h][w] == 3 {
					print!("C ");
				}
				// Blank space
				else if map[h][w] == 0 {
					print!("  ");
				}
				// Tiles
				else if map[h][w] == 1 {
					print!(". ");
				}
				// Walls
				else if map[h][w] == 2 || map[h][w] == 5 {
					print!("+ ");
				}
				// Upstairs
				else if map[h][w] == 3{
					print!("U ");
				}
				// Downstairs
				else if map[h][w] == 4{
					print!("D ");
				}	
				else if map[h][w] == 6{
					print!("S ");
				}				
			}
			println!("");
		}
	}
}