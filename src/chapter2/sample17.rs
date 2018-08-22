use std::f64::consts::PI;
use stdweb::web::CanvasRenderingContext2d;
use Point;

pub fn draw(context: &CanvasRenderingContext2d) {
    context.set_stroke_style_color("#000");

    let mut points = vec![];

    /*高w，宽x*/
    let (w, x) = (200.0, 800.0);
    /*L=0 环中心无空洞 */
    let l = 0.0;

    /*计算原型图案的坐标 */
    let mut a = 0.0;
    while a <= 2.0 * PI {
        let d = 32.0 * (4.0 / 5.0 + 1.0 / 5.0 * (18.0 * a).sin());
        let t = d * (1.0 + (6.0 * a).sin());
        points.push(Point {
            x: t * a.cos() * 1.2,
            y: t * a.sin(),
        });
        a += PI / 120.0;
    }

    context.begin_path();

    /*变换*/
    let transform = |x1: f64, y1: f64| -> (f64, f64) {
        let th = 2.0 * PI * (x - x1) / x;
        let x2 = (l + y1) * th.cos() + 320.0;
        let y2 = (l + y1) * th.sin() + 200.0;
        (x2, y2)
    };

    let mut py = w / 4.0;
    while py <= 3.0 * w / 4.0 {
        let mut px = x / 16.0;
        while px <= x - x / 16.0 {
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
            px += x / 8.0;
        }
        py += w / 2.0;
    }

    context.stroke();
}
