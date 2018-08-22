use std::f64::consts::PI;
use stdweb::web::CanvasRenderingContext2d;
use Point;

pub fn draw(context: &CanvasRenderingContext2d) {
    context.set_stroke_style_color("#000");

    let mut points1 = [Point::new(); 120];
    let mut points2 = [Point::new(); 120];

    let nn = 20.0;

    let r = 180.0;
    let mut a = 0.0f64;
    for i in 0..points1.len() {
        points1[i].x = r * a.cos();
        points1[i].y = -r * a.sin();
        a += PI / 60.0;
    }

    a = 0.0;
    let m = 4.0;
    let k = 30.0;
    let r = 90.0;
    let l = 2.0 * r * (PI / m).sin();
    let dd = l / k;
    let mut j = 0;
    while a <= 2.0 * PI - 2.0 * PI / m {
        let aa = a + PI / 2.0;
        let x0 = r * aa.sin();
        let y0 = r * aa.cos();
        for i in 0..k as i32 {
            points2[j].x = x0 + i as f64 * dd * (aa + PI / 2.0 + PI / m).sin();
            points2[j].y = y0 + i as f64 * dd * (aa + PI / 2.0 + PI / m).cos();
            j += 1;
        }
        a += 2.0 * PI / m;
    }

    context.begin_path();

    for n in 0..=nn as i32 {
        let px = 60.0 * n as f64 / nn + 320.0;
        let mut start = Point::new();
        for i in 0..120 {
            let x = (points2[i].x - points1[i].x) / nn * n as f64 + points1[i].x;
            let y = (points2[i].y - points1[i].y) / nn * n as f64 + points1[i].y;
            let x = x + px;
            let y = y + 200.0;
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

    context.stroke();
}
