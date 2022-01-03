# Slime Rampage

In this game, the user plays as a slime trying to make its way to the end of a dungeon to kill the boss. There are various abilities and upgrades that can be found along the way and permanent upgrades that can be unlocked by playing enough. The user can move with 'WASD', melee with 'space', and use various abilities with the left mouse click. 

# Rust Setup and Running the Game

The game can be run with 'cargo run'. Requires Rust, SDL, and C++ to be installed and linked to the user's path.

Windows (assuming Rust installed through rustup)
Download the following files: 
- Download the SDL2-devel-2.0.16-VC.zip
- Download the SDL2_image-devel-2.0.5-VC.zip
- Download the SDL2_mixer-devel-2.0.4-VC.zip
- Download the SDL2_ttf-devel-2.0.15-VC.zip
Locate your install of rustup.
- Mine was 'C:\Users\{username}\.rustup'
- Navigate to the following path (replacing 'C:\Users\{username}\.rustup' with your rustup location): 'C:\Users\{username}\.rustup\toolchains\{current_toolchain}\lib\rustlib\x86_64-pc-windows-msvc\lib' where 'current_toolchain' will likely be the most recently modified folder with the name stable in it. 
  - I think the process is similar for those who have rust installed through different means. Basing off of the https://github.com/Rust-SDL2/rust-sdl2 repo, the folder path might be C:\Program Files\Rust\lib\rustlib\x86_64-pc-windows-msvc\lib though I cannot confirm.
- Add the path above to your environment variables. Ensure the variable name is 'LIBRARY_PATH'. 
- From each .zip, navigate roughly to {file name}\lib\x64 and copy all contents into the path mentioned above. 
  - Copy 'SDL2.dll', 'SDL2_image.dll', 'SDL2_mixer.dll', and 'SDL2_ttf.dll' found within their respective .zips to your project folder placed in the same location as 'Cargo.toml'
- Navigate to the SlimeRampage game contents folder and call 'cargo run' to start the game. 
