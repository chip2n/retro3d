mod math;

use bresenham::Bresenham;
use math::Vector;
use minifb::{Key, Scale, Window, WindowOptions};
use std::time::Instant;

const WIDTH: usize = 240;
const HEIGHT: usize = 160;

const SPEED: f32 = 40.0;

/// Player's turn rate in radians per second
const TURN_RATE: f32 = 1.5 * math::PI;

type Buffer = [u32];
type Point = Vector;
type Map = Vec<Line>;

struct Player {
    position: Point,
    look_dir: Vector,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

fn build_map() -> Map {
    vec![Line {
        start: Vector::new(40.0, 20.0),
        end: Point::new(80.0, 20.0),
    }]
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
        draw_map(&mut buffer, &map);
        draw_player(&mut buffer, &player);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn clear(buffer: &mut Buffer) {
    for i in buffer.iter_mut() {
        *i = 0xFF0000FF;
    }
}

fn draw_map(buffer: &mut Buffer, map: &Map) {
    for line in map.iter() {
        draw_line(buffer, &line);
    }
}

fn draw_player(buffer: &mut Buffer, player: &Player) {
    draw_arrow(buffer, player.position, player.look_dir);
    *pixel(
        buffer,
        player.position.x as usize,
        player.position.y as usize,
    ) = 0xFFFFFF;
}

fn draw_arrow(buffer: &mut Buffer, origin: Point, direction: Vector) {
    let end = origin + direction * 10.0;

    let line = Line { start: origin, end };

    let left_line = Line {
        start: end,
        end: end - 6.0 * rotate_vector(direction, (45.0 as f32).to_radians()),
    };

    let right_line = Line {
        start: end,
        end: end - 6.0 * rotate_vector(direction, (-45.0 as f32).to_radians()),
    };

    draw_line(buffer, &line);
    draw_line(buffer, &left_line);
    draw_line(buffer, &right_line);
}

fn draw_line(buffer: &mut Buffer, line: &Line) {
    let start = (line.start.x as isize, line.start.y as isize);
    let end = (line.end.x as isize, line.end.y as isize);

    let coords = Bresenham::new(start, end);
    for (x, y) in coords {
        *pixel(buffer, x as usize, y as usize) = 0x00FF00;
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
