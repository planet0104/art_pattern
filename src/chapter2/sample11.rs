use std::f64::consts::PI;
use stdweb::web::CanvasRenderingContext2d;
use Point;

pub fn draw(context: &CanvasRenderingContext2d) {
    context.set_stroke_style_color("#000");

    let mut points1 = [Point::new(); 120];
    let mut points2 = [Point::new(); 120];

    let nn = 35.0;
    let r = 35.0;

    let mut a = 0.0f64;
    for i in 0..points1.len() {
        points1[i].x = 1.1 * (r / 5.0 * (8.0 * a).sin() + r * (2.0 * a).sin()) * a.cos();
        points1[i].y = 0.85 * (r / 5.0 * (8.0 * a).sin() + r * (2.0 * a).sin()) * a.sin();
        a += PI / 60.0;
    }

    a = 0.0;
    for i in 0..points2.len() {
        points2[i].x = (r / 5.0 * (8.0 * a).sin() + r * (2.0 * a).sin()) * a.cos();
        points2[i].y = (r / 5.0 * (8.0 * a).sin() + r * (2.0 * a).sin()) * a.sin();
        a += PI / 60.0;
    }

    let mut n = -1.0;
    context.begin_path();

    for px in (170..=470).step_by(60) {
        for py in (50..=350).step_by(60) {
            n += 1.0;
            let mut start = Point::new();
            for i in 0..120 {
                let x = (points2[i].x - points1[i].x) / nn * n + points1[i].x;
                let y = (points2[i].y - points1[i].y) / nn * n * 2.0 + points1[i].y;
                let x = x + px as f64;
                let y = y + py as f64;
                if i == 0 {
                    context.move_to(x, y);
                    start.x = x;
                    start.y = y;
                } else {
                    context.line_to(x, y);
                }
            }
            context.line_to(start.x, start.y);
        }
    }

    context.stroke();
}
