use crate::Point;

type OutCode = u8;

const INSIDE: OutCode = 0b0000;
const LEFT: OutCode = 0b0001;
const RIGHT: OutCode = 0b0011;
const BOTTOM: OutCode = 0b0111;
const TOP: OutCode = 0b1111;

pub struct Rect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

fn compute_outcode(x: f32, y: f32, rect: &Rect) -> u8 {
    let mut code = INSIDE;

    if x < rect.left {
        code = code | LEFT;
    }

    if x > rect.right {
        code = code | RIGHT;
    }

    if y < rect.top {
        code = code | TOP;
    }

    if y > rect.bottom {
        code = code | BOTTOM;
    }

    code
}

fn compute_outcode_point(point: Point, rect: &Rect) -> u8 {
    compute_outcode(point.x, point.y, rect)
}

fn calculate_intersection(p1: Point, p2: Point, rect: &Rect, clip_to: u8) -> Point {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;

    let slope_y = dx / dy; // slope to use for possibly-vertical lines
    let slope_x = dy / dx; // slope to use for possibly-horizontal lines

    if clip_to & TOP == TOP {
        return Point {
            x: p1.x + slope_y * (rect.top - p1.y),
            y: rect.top,
        };
    }

    if clip_to & BOTTOM == BOTTOM {
        return Point {
            x: p1.x + slope_y * (rect.bottom - p1.y),
            y: rect.bottom,
        };
    }

    if clip_to & RIGHT == RIGHT {
        return Point {
            x: rect.right,
            y: p1.y + slope_x * (rect.right - p1.x),
        };
    }

    if clip_to & LEFT == LEFT {
        return Point {
            x: rect.left,
            y: p1.y + slope_x * (rect.left - p1.x),
        };
    }

    panic!("Incorrect clipping: {}", clip_to);
}

pub fn clip_line(p1: Point, p2: Point, rect: &Rect) -> Option<(Point, Point)> {
    let mut outcode_p1 = compute_outcode_point(p1, rect);
    let mut outcode_p2 = compute_outcode_point(p2, rect);
    let mut p1 = p1;
    let mut p2 = p2;
    let mut accept = false;

    loop {
        // both endpoints are within the clipping region
        if (outcode_p1 | outcode_p2) == INSIDE {
            accept = true;
            break;
        }

        // both endpoints share an excluded region - cannot be within clipping region
        if (outcode_p1 & outcode_p2) != 0 {
            break;
        }

        // endpoints are in different regions - line is partially within clipping region
        let outcode = if outcode_p1 != INSIDE {
            outcode_p1
        } else {
            outcode_p2
        };

        let p = calculate_intersection(p1, p2, rect, outcode);

        // update point after clipping and recalculate outcode
        if outcode == outcode_p1 {
            p1 = p;
            outcode_p1 = compute_outcode_point(p1, rect);
        } else {
            p2 = p;
            outcode_p2 = compute_outcode_point(p2, rect);
        }
    }

    if accept {
        Some((p1, p2))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_line_inside() {
        let p1 = Point { x: 10.0, y: 10.0 };
        let p2 = Point { x: 20.0, y: 20.0 };
        let rect = Rect {
            left: 0.0,
            right: 30.0,
            top: 0.0,
            bottom: 30.0,
        };

        let (new_p1, new_p2) = clip_line(p1, p2, &rect).unwrap();
        assert_eq!(p1, new_p1);
        assert_eq!(p2, new_p2);
    }

    #[test]
    fn clip_line_outside() {
        let p1 = Point { x: 10.0, y: 10.0 };
        let p2 = Point { x: 20.0, y: 20.0 };
        let rect = Rect {
            left: 30.0,
            right: 50.0,
            top: 0.0,
            bottom: 30.0,
        };

        let result = clip_line(p1, p2, &rect);
        assert_eq!(None, result);
    }

    #[test]
    fn clip_line_intersected() {
        let p1 = Point { x: 10.0, y: 10.0 };
        let p2 = Point { x: 30.0, y: 30.0 };
        let rect = Rect {
            left: 0.0,
            right: 20.0,
            top: 0.0,
            bottom: 20.0,
        };

        let (new_p1, new_p2) = clip_line(p1, p2, &rect).unwrap();
        assert_eq!(p1, new_p1);
        assert_eq!(Point { x: 20.0, y: 20.0 }, new_p2);
    }
}
