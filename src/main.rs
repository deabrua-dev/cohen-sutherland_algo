fn main() {
    let mut a = Point { x: 10, y: 270 };
    let mut b = Point { x: 300, y: 0 };
    let rect = Rectangle { top: 240, bottom: 50, left: 30, right: 220 };
    println!("{}", cohen_sutherland(rect, &mut a, &mut b));
    println!("Point A x: {} y: {}", a.x, a.y);
    println!("Point B x: {} y: {}", b.x, b.y);
}

const INSIDE: i32 = 0;
const TOP: i32 = 1;
const BOTTOM: i32 = 2;
const LEFT: i32 = 4;
const RIGHT: i32 = 8;

struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
}

fn compute(rect : &Rectangle, point: &Point) -> i32 {
    let mut code = INSIDE;
    if point.x < rect.left {
        code |= LEFT;
    } else if point.x > rect.right {
        code |= RIGHT;
    }
    if point.y < rect.bottom {
        code |= BOTTOM;
    } else if point.y > rect.top {
        code |= TOP;
    }
    code
}

fn cohen_sutherland(rect : Rectangle, point_1: &mut Point, point_2:  &mut Point) -> bool {
    let mut outcode_0 = compute(&rect, &point_1);
    let mut outcode_1 = compute(&rect, &point_2);
    let mut accept = false;
    loop {
        if !((outcode_0 | outcode_1) != 0) {
            accept = true;
            break;
        } else if (outcode_0 & outcode_1) != 0 {
            break;
        } else {
            let mut point = Point { x: 0, y: 0 };
            let outcode_out = if outcode_1 > outcode_0 {
                outcode_1
            } else {
                outcode_0
            };
            
            if (outcode_out & TOP) != 0 {
                point.x = point_1.x + (point_2.x - point_1.x) * (rect.top - point_1.y) / (point_2.y - point_1.y);
                point.y = rect.top;
            } else if (outcode_out & BOTTOM) != 0 {
                point.x = point_1.x + (point_2.x - point_1.x) * (rect.bottom - point_1.y) / (point_2.y - point_1.y);
                point.y = rect.bottom;
            } else if (outcode_out & RIGHT) != 0 {
                point.y = point_1.y + (point_2.y - point_1.y) * (rect.right - point_1.x) / (point_2.x - point_1.x);
                point.x = rect.right;
            } else if (outcode_out & LEFT) != 0 {
                point.y = point_1.y + (point_2.y - point_1.y) * (rect.left - point_1.x) / (point_2.x - point_1.x);
                point.x = rect.left;
            }

            if outcode_out == outcode_0 {
                point_1.x = point.x;
                point_1.y = point.y;
                outcode_0 = compute(&rect, &point_1);
            } else {
                point_2.x = point.x;
                point_2.y = point.y;
                outcode_1 = compute(&rect, &point_2);
            }
        }
    }
    accept
}
