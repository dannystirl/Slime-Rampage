use crate::main::*;
use sdl2::image::LoadTexture;
use rogue_sdl::SDLCore;
use rogue_sdl::Game;
mod enemy;
mod player;
mod ranged_attack;
mod credits;

use std::collections::HashSet;
use rand::Rng;
use crate::enemy::*;
use crate::ranged_attack::*;
use crate::player::*;