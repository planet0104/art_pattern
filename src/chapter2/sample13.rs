use std::f64::consts::PI;
use stdweb::web::CanvasRenderingContext2d;
use Point;

pub fn draw(context: &CanvasRenderingContext2d) {
    context.set_stroke_style_color("#000");

    let mut points1 = [Point::new(); 120];
    let mut points2 = [Point::new(); 120];

    let r = 180.0;
    let mut a = 0.0f64;
    for i in 0..points1.len() {
        points1[i].x = r * a.cos();
        points1[i].y = -r * a.sin();
        a += PI / 60.0;
    }

    a = 0.0;
    let r = 100.0;
    for i in 0..points2.len() {
        let d = r * (1.0+1.0/5.0/*控制花瓣凹度*/*(12.0*a).sin());
        let t = d * (1.0 / 2.0 + 1.0 / 2.0 * (4.0 * a).sin());
        points2[i].x = t * a.cos();
        points2[i].y = -t * a.sin();
        a += PI / 60.0;
    }

    context.begin_path();
    /*变化的次数*/
    let nn = 25.0;
    for n in 0..=nn as i32 {
        let mut start = Point::new();
        for i in 0..120 {
            let x = (points2[i].x - points1[i].x) / nn * n as f64 + points1[i].x;
            let y = (points2[i].y - points1[i].y) / nn * n as f64 + points1[i].y;
            let x = x + 320.0;
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
