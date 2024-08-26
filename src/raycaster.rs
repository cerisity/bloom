use crate::player::Player;
use crate::map::Map;

pub struct Raycaster {
    fov: usize,
    focal_length: f32,
    max_distance: f32,
}

impl Raycaster {
    pub fn new(fov: usize, focal_length: f32, max_distance: f32) -> Self {
        Raycaster { fov, focal_length, max_distance }
    }

    pub fn cast_rays(&self, player: &Player, map: &Map) -> Vec<String> {
        let mut view = vec![String::new(); 20];
        
        for x in 0..self.fov {
            let angle = player.angle - self.focal_length / 2.0 + x as f32 * self.focal_length / self.fov as f32;
            let (hit, distance) = self.cast_ray(player.x, player.y, angle, map);
            
            if hit {
                let height = (20.0 / distance) as usize;
                let wall_top = 10 - height / 2;
                let wall_bottom = 10 + height / 2;
                
                for y in 0..20 {
                    if y < wall_top {
                        view[y].push(' ');
                    } else if y > wall_bottom {
                        view[y].push('.');
                    } else {
                        view[y].push('X');
                    }
                }
            } else {
                for y in 0..20 {
                    view[y].push(' ');
                }
            }
        }
        
        view
    }

    fn cast_ray(&self, x: f32, y: f32, angle: f32, map: &Map) -> (bool, f32) {
        let mut distance = 0.0;
        while distance < self.max_distance {
            let check_x = (x + angle.cos() * distance) as usize;
            let check_y = (y + angle.sin() * distance) as usize;
            
            if map.is_wall(check_x, check_y) {
                return (true, distance);
            }
            
            distance += 0.1;
        }
        (false, self.max_distance)
    }
}