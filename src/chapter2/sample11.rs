use std::f64::consts::PI;
use stdweb::web::CanvasRenderingContext2d;
use Point;

pub fn draw(context: &CanvasRenderingContext2d) {
    context.set_stroke_style_color("#000");

    let mut points1 = vec![];
    let mut points2 = vec![];

    let mut n = -1.0;
    let nn = 35.0;
    let r = 35.0;

    let mut a = 0.0f64;
    while a < 2.0 * PI {
        points1.push(Point {
            x: 1.1 * (r / 5.0 * (8.0 * a).sin() + r * (2.0 * a).sin()) * a.cos(),
            y: 0.85 * (r / 5.0 * (8.0 * a).sin() + r * (2.0 * a).sin()) * a.sin(),
        });
        a += PI / 60.0;
    }

    let mut a = 0.0f64;
    while a < 2.0 * PI {
        points2.push(Point {
            x: (r / 5.0 * (8.0 * a).sin() + r * (2.0 * a).sin()) * a.cos(),
            y: (r / 5.0 * (8.0 * a).sin() + r * (2.0 * a).sin()) * a.sin(),
        });
        a += PI / 60.0;
    }

    context.begin_path();

    for px in (170..=470).step_by(60) {
        for py in (50..=350).step_by(60) {
            n += 1.0;
            let (mut bx, mut by) = (0.0, 0.0);
            for i in 0..points1.len() {
                let x = (points2[i].x - points1[i].x) / nn * n + points1[i].x;
                let y = (points2[i].y - points1[i].y) / nn * n * 2.0 + points1[i].y;
                let x = x + px as f64;
                let y = y + py as f64;
                if i == 0 {
                    context.move_to(x, y);
                    bx = x;
                    by = y;
                } else {
                    context.line_to(x, y);
                }
            }
            context.line_to(bx, by);
        }
    }

    context.stroke();
}
