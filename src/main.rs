mod player;
mod map;
mod raycaster;

use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use player::Player;
use map::Map;
use raycaster::Raycaster;

fn main() {
    let mut player = Player::new(2.0, 2.0, 0.0);
    let map = Map::new(10, 10);
    let raycaster = Raycaster::new(60, 0.66, 10.0);

    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("Player position: ({:.2}, {:.2}), Angle: {:.2}", player.x, player.y, player.angle);
        println!("Enter move (w/a/s/d) or q to quit:");

        let view = raycaster.cast_rays(&player, &map);
        for row in view {
            println!("{}", row);
        }
        
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "w" => player.move_forward(0.5, &map),
            "s" => player.move_backward(0.5, &map),
            "a" => player.rotate(-0.2),
            "d" => player.rotate(0.2),
            "q" => break,
            _ => {}
        }

        thread::sleep(Duration::from_millis(5));
    }
}
