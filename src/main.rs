mod math;

use bresenham::Bresenham;
use math::Vector;
use minifb::{Key, Scale, Window, WindowOptions};
use std::time::Instant;

const WIDTH: usize = 240;
const HEIGHT: usize = 160;

const SPEED: f32 = 40.0;

/// Player's turn rate in radians per second
const TURN_RATE: f32 = 1.0 * math::PI;

type Buffer = [u32];
type Point = Vector;

struct Map {
    walls: Vec<Line>,
    width: usize,
    height: usize,
}

struct Player {
    position: Point,
    look_dir: Vector,
}

#[derive(Copy, Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn translate(&self, offset: Vector) -> Self {
        Line {
            start: self.start + offset,
            end: self.end + offset,
        }
    }

    fn rotate(&self, angle: f32, origin: Point) -> Self {
        Line {
            start: rotate_vector(self.start - origin, angle) + origin,
            end: rotate_vector(self.end - origin, angle) + origin,
        }
    }

    fn scale(&self, scale: f32) -> Self {
        Line {
            start: self.start * scale,
            end: self.end * scale,
        }
    }
}

fn build_map() -> Map {
    Map {
        width: 100,
        height: 100,
        walls: vec![Line {
            start: Vector::new(40.0, 20.0),
            end: Point::new(80.0, 20.0),
        }],
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut player = Player {
        position: Vector::new(50.0, 50.0),
        look_dir: Vector::new(0.0, -1.0),
    };
    let map = build_map();

    let mut last_time = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current_time = Instant::now();
        let dt = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;

        if window.is_key_down(Key::F) {
            player.position += player.look_dir * SPEED * dt;
        }

        if window.is_key_down(Key::S) {
            player.position -= player.look_dir * SPEED * dt;
        }

        if window.is_key_down(Key::R) {
            player.look_dir = rotate_vector(player.look_dir, -TURN_RATE * dt);
        }

        if window.is_key_down(Key::T) {
            player.look_dir = rotate_vector(player.look_dir, TURN_RATE * dt)
        }

        clear(&mut buffer);

        let center = Vector::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);
        let map_offset = center - player.position;

        // render world
        {
            let rotation = rotation_between(Vector::up(), player.look_dir);
            let projected_map = map
                .walls
                .iter()
                .map(|line| line.rotate(-rotation, player.position))
                .map(|line| line.translate(map_offset));
            let culled_map = cull(projected_map);
            draw_map(&mut buffer, culled_map.into_iter());
            *pixel(&mut buffer, center.x as usize, center.y as usize) = 0xFFFFFF;
        }

        // render minimap
        {
            let scale = 0.5;
            draw_rect(
                &mut buffer,
                0,
                0,
                (map.width as f32 * scale) as usize,
                (map.height as f32 * scale) as usize,
            );
            let scaled_map = map.walls.iter().map(|line| line.scale(scale));
            draw_map(&mut buffer, scaled_map);

            let player_pos = player.position * scale;
            draw_line(&mut buffer, player_pos, player_pos + player.look_dir * 5.0);
            *pixel(&mut buffer, player_pos.x as usize, player_pos.y as usize) = 0xFFFFFF;
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn clear(buffer: &mut Buffer) {
    for i in buffer.iter_mut() {
        *i = 0xFF0000FF;
    }
}

fn draw_map(buffer: &mut Buffer, map: impl Iterator<Item = Line>) {
    for line in map {
        draw_line(buffer, line.start, line.end);
    }
}

fn draw_arrow(buffer: &mut Buffer, origin: Point, direction: Vector) {
    let base_end = origin + direction * 10.0;

    let left_wing_end = base_end - 6.0 * rotate_vector(direction, (45.0 as f32).to_radians());
    let right_wing_end = base_end - 6.0 * rotate_vector(direction, (-45.0 as f32).to_radians());

    draw_line(buffer, origin, base_end);
    draw_line(buffer, base_end, left_wing_end);
    draw_line(buffer, base_end, right_wing_end);
}

fn draw_line(buffer: &mut Buffer, start: Point, end: Point) {
    let start = (start.x as isize, start.y as isize);
    let end = (end.x as isize, end.y as isize);

    let coords = Bresenham::new(start, end)
        .filter(|(x, y)| (0..WIDTH as isize).contains(x) && (0..HEIGHT as isize).contains(y));

    for (x, y) in coords {
        *pixel(buffer, x as usize, y as usize) = 0x00FF00;
    }
}

fn draw_rect(buffer: &mut Buffer, x: usize, y: usize, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            *pixel(buffer, x, y) = 0x000000;
        }
    }
}

fn pixel(buffer: &mut Buffer, x: usize, y: usize) -> &mut u32 {
    &mut buffer[(y * WIDTH) + x]
}

fn rotate_vector(v: Vector, angle: f32) -> Vector {
    let (x1, y1) = (v.x, v.y);

    let x2 = angle.cos() * x1 - angle.sin() * y1;
    let y2 = angle.sin() * x1 + angle.cos() * y1;

    Vector::new(x2, y2)
}

/// Calculate rotation between two normalized vectors
fn rotation_between(v1: Vector, v2: Vector) -> f32 {
    v2.y.atan2(v2.x) - v1.y.atan2(v1.x)
}

/// Cull lines that are behind the "camera" (i.e. center of viewport)
fn cull(lines: impl Iterator<Item = Line>) -> Vec<Line> {
    let center_y = HEIGHT as f32 / 2.0;
    lines
        .filter(|line| line.start.y <= center_y || line.end.y <= center_y)
        .map(|line| {
            let k = (line.start.y - line.end.y) / (line.start.x - line.end.x);
            let m = line.start.y - k * line.start.x;

            let y1 = line.start.y;
            let y2 = line.end.y;

            if y1 > center_y {
                let new_y = center_y;
                let new_x = (new_y - m) / k;
                Line {
                    start: Point::new(new_x, new_y),
                    end: line.end,
                }
            } else if y2 > center_y {
                let new_y = center_y;
                let new_x = (new_y - m) / k;
                Line {
                    start: line.start,
                    end: Point::new(new_x, new_y),
                }
            } else {
                line
            }
        })
        .collect()
}
