// took knowledge big time from this guide, legend: https://lodev.org/cgtutor/raycasting.html#Untextured_Raycaster_

use std::time::{Instant, Duration};
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::render::Canvas;

const MAP_W: usize = 8;
const MAP_H: usize = 8;
const SCREEN_W: u16 = 640;
const SCREEN_H: u16 = 480;
const WORLD_MAP: [[u8; MAP_W]; MAP_H] = [
    [1,1,1,1,1,1,1,1],
    [1,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,1],
    [1,0,2,0,2,0,0,1],
    [1,0,2,0,2,0,0,1],
    [1,0,2,2,2,0,0,1],
    [1,1,1,1,1,1,1,1]
];

struct Player {
    pos_x: f32,
    pos_y: f32,
    dir_x: f32,
    dir_y: f32,
    plane_x: f32,
    plane_y: f32,
}

fn main() -> Result<(), String> {

    // Window handling
    // also this video mega huge: https://youtu.be/Qz6ed3KBSc0/
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let window = video.window("bloom.", SCREEN_W as u32, SCREEN_H as u32)
        .position_centered()
        .resizable()
        .build()
        .expect("Something went wrong with window initialisation");
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Canvas creation failed");

    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();
    canvas.present();

    // Position Data
    let player = Player {
        // Pos is the players position
        pos_x: 3.0,
        pos_y: 3.0,
        // Dir is the players direction
        dir_x: -1.0,
        dir_y: 0.0,
        // Plane is the 2d plane that functions as the camera
        plane_x: 0.0,
        plane_y: 0.66
    }; 
    
    // Frame Time
    let mut frame_time = Instant::now();
    let mut old_time = Instant::now();
    let mut delta_time: Duration;
    let mut time_mod: f32 = 0.0;
    
    // Gameloop
    let done = false;
    while !done {
        raycast(&player, &mut canvas);
        frame_time = Instant::now();
        delta_time = frame_time.duration_since(old_time);
        old_time = frame_time;
        time_mod = delta_time.as_secs_f32();
        let fps = 1.0 / time_mod;
    };
    Ok(())
}

fn raycast(player: &Player, canvas: &mut Canvas<Window>) {
    for i in 0..SCREEN_W {
        // Ray pos and direction calculations
        let camera_x: f32 = 2.0 * i as f32 / SCREEN_W as f32 - 1.0;
        let ray_dir_x = player.dir_x + player.plane_x * camera_x;
        let ray_dir_y = player.dir_y + player.plane_y * camera_x;
        
        // Which box of the map the ray is currently in
        let mut map_x = player.pos_x as i8;
        let mut map_y = player.pos_y as i8;

        // Length of ray from current pos to next X/Y side of map square
        let (mut side_dist_x, mut side_dist_y): (f32,f32);

        // Length of ray from X/Y side to next X/Y side
        let delta_dist_x = if ray_dir_x == 0.0 {1e30} else {(1.0 / ray_dir_x).abs()};
        let delta_dist_y = if ray_dir_y == 0.0 {1e30} else {(1.0 / ray_dir_y).abs()};
        let perp_wall_dist: f32;

        // Step X/Y direction (+1/-1)
        let (step_x, step_y): (i8,i8);

        let mut hit: bool = false; // is wall detected
        let mut side: bool = true; // true = Y side hit, false = X side hit, assigning true value initially here just to shut the compiler up
        
        // Step calc
        if ray_dir_x < 0.0 {
            step_x = -1;
            side_dist_x = (player.pos_x - map_x as f32) * delta_dist_x;
        }
        else {
            step_x = 1;
            side_dist_x = (map_x as f32 + 1.0 - player.pos_x) * delta_dist_x;
        }
        if ray_dir_y < 0.0 {
            step_y = -1;
            side_dist_y = (player.pos_y - map_y as f32) * delta_dist_y;
        }
        else {
            step_y = 1;
            side_dist_y = (map_y as f32 + 1.0 - player.pos_y) * delta_dist_y;
        }

        // DDA (digital differential analyser)
        while !hit {
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x as i8;
                side = false;
            }
            else {
                side_dist_y += delta_dist_y;
                map_y += step_y as i8;
                side = true;
            }
            if WORLD_MAP[map_x as usize][map_y as usize] > 0 { hit = true };
        }

        // Calculate wall dist
        if side { perp_wall_dist = side_dist_x - delta_dist_x }
        else { perp_wall_dist = side_dist_y - delta_dist_y }

        // Calculate line height to draw on screen
        let line_height = (SCREEN_H as f32 / perp_wall_dist) as i32;

        // Calculate lowest and highest pixel to fill in current stripe
        let h: i32 = SCREEN_H as i32;
        let mut draw_start = -line_height / 2 + h / 2;
        if draw_start < 0 { draw_start = 0 };
        let mut draw_end = line_height / 2 + h / 2;
        if draw_end >= h { draw_end = h - 1};

        // Color
        let mut max: u8 = 255;
        if side { max = 150 };
        let color;
        match WORLD_MAP[map_x as usize][map_y as usize] {
            1 => color = Color::RGB(max,0,0),
            2 => color = Color::RGB(0,max,0),
            3 => color = Color::RGB(0,0,max),
            _ => color = Color::RGB(max,max,max)
        }

        canvas.set_draw_color(color);
        let _ = canvas.draw_line(Point::new(i as i32, draw_start), Point::new(i as i32, draw_end));
    }
}
