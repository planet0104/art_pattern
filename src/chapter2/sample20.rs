use std::f64::consts::PI;
use stdweb::web::CanvasRenderingContext2d;
use Point;

pub fn draw(context: &CanvasRenderingContext2d) {
    context.set_stroke_style_color("#000");

    let mut points = vec![];

    let (x, w, l) = (660.0, 198.0, 0.0);

    /*计算原型图案的坐标 */
    let mut a = 0.0;
    while a <= 2.0 * PI {
        let d = w / 6.0 * (4.0 / 5.0 + 1.0 / 5.0 * (18.0 * a).sin());
        let r = d * (1.0 / 2.0 + 1.0 / 2.0 * (6.0 * a).sin());
        points.push(Point {
            x: 1.4 * r * a.cos(),
            y: 1.4 * r * a.sin(),
        });
        a += PI / 120.0;
    }

    context.begin_path();

    /*变换*/
    let transform = |x1: f64, y1: f64| -> (f64, f64) {
        let th = 2.0 * PI * (x - x1) / x;
        let x2 = (l + y1) * th.cos() + 320.0;
        let y2 = (l + y1) * th.sin() + 210.0;
        (x2, y2)
    };

    let mut px = w / 6.0;
    while px <= x - w / 6.0 {
        let mut py = w / 6.0;
        while py <= w - w / 16.0 {
            let mut start = Point::new();
            for i in 0..points.len() {
                let (x2, y2) = transform(points[i].x + px, points[i].y + py);
                if i == 0 {
                    context.move_to(x2, y2);
                    start.x = x2;
                    start.y = y2;
                } else {
                    context.line_to(x2, y2);
                }
            }
            context.line_to(start.x, start.y);
            py += w / 3.0;
        }
        px += x / 10.0;
    }

    context.stroke();
}
