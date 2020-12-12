use bresenham::Bresenham;
use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 240;
const HEIGHT: usize = 160;

type Point = (usize, usize);

struct Player {
    position: Point,
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

    let player = Player { position: (50, 50) };
    let map = build_map();

    while window.is_open() && !window.is_key_down(Key::Escape) {
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
    *pixel(buffer, player.position) = 0x00FF00;
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
