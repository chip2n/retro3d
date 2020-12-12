use std::time::Instant;

use bresenham::Bresenham;
use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 240;
const HEIGHT: usize = 160;

type Buffer = [u32];
type Point = (usize, usize);

struct Player {
    position: Point,
    rotation: f32,
}

struct Line {
    start: Point,
    end: Point,
}

fn build_map() -> Vec<Line> {
    vec![Line {
        start: (40, 20),
        end: (80, 20),
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
        position: (50, 50),
        rotation: 20.0,
    };
    let map = build_map();

    let mut last_time = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current_time = Instant::now();
        let dt = current_time.duration_since(last_time).as_millis() as f32;
        last_time = current_time;

        if window.is_key_down(Key::R) {
            player.rotation -= 1.0 / dt;
        }

        if window.is_key_down(Key::T) {
            player.rotation += 1.0 / dt;
        }

        for i in buffer.iter_mut() {
            *i = 0xFF0000FF;
        }

        for line in map.iter() {
            draw_line(&mut buffer, &line);
        }

        draw_player(&mut buffer, &player);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn draw_player(buffer: &mut [u32], player: &Player) {
    draw_arrow(buffer, player.position, player.rotation);
    *pixel(buffer, player.position) = 0xFFFFFF;
}

fn draw_arrow(buffer: &mut Buffer, start: Point, rot: f32) {
    let end = (start.0, start.1 - 10);
    let origin_line = Line { start, end };
    let left_line = Line {
        start: end,
        end: (end.0 - 4, end.1 + 4),
    };
    let right_line = Line {
        start: end,
        end: (end.0 + 4, end.1 + 4),
    };

    draw_line(buffer, &rotate_line(&origin_line, rot, start));
    draw_line(buffer, &rotate_line(&left_line, rot, start));
    draw_line(buffer, &rotate_line(&right_line, rot, start));
}

fn draw_line(buffer: &mut [u32], line: &Line) {
    let start = (line.start.0 as isize, line.start.1 as isize);
    let end = (line.end.0 as isize, line.end.1 as isize);

    let coords = Bresenham::new(start, end);
    for (x, y) in coords {
        *pixel(buffer, (x as usize, y as usize)) = 0x00FF00;
    }
}

fn pixel(buffer: &mut [u32], point: Point) -> &mut u32 {
    &mut buffer[(point.1 as usize * WIDTH) + point.0 as usize]
}

fn rotate_line(line: &Line, angle: f32, origin: Point) -> Line {
    Line {
        start: rotate_point(line.start, angle, origin),
        end: rotate_point(line.end, angle, origin),
    }
}

fn rotate_point(point: Point, angle: f32, origin: Point) -> Point {
    let (x, y) = (point.0 as f32, point.1 as f32);
    let (origin_x, origin_y) = (origin.0 as f32, origin.1 as f32);

    let x1 = x - origin_x;
    let y1 = y - origin_y;

    let x2 = angle.cos() * x1 - angle.sin() * y1;
    let y2 = angle.sin() * x1 + angle.cos() * y1;

    (
        (origin_x + x2).round() as usize,
        (origin_y + y2).round() as usize,
    )
}
