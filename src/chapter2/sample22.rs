use std::f64::consts::PI;
use stdweb::web::CanvasRenderingContext2d;
use Point;

pub fn draw(context: &CanvasRenderingContext2d) {
    context.set_stroke_style_color("#000");

    let mut points = vec![];
    let r = 45.0;

    /*花形单元 */
    let mut a = 0.0;
    while a <= 2.0 * PI {
        let d = r * (1.0 + 1.0 / 5.8 * (12.0 * a).sin());
        let t = d * (1.0 / 2.0 + 1.0 / 2.0 * (4.0 * a).sin());
        points.push(Point {
            x: t * a.cos(),
            y: t * a.sin(),
        });
        a += PI / 60.0;
    }

    context.begin_path();

    /*单元配置和绘图 */
    for px in (170..=470).step_by(60).map(|px| px as f64) {
        for py in (50..=350).step_by(60).map(|py| py as f64) {
            let (mut bx, mut by) = (0.0, 0.0);
            for i in 0..points.len() {
                let (x, y) = (points[i].x + px, points[i].y + py);
                /*变换 这里如果改为 x2=x; 或者 y2=y; 则只变换x或者y坐标。 */
                let x2 = 320.0 - 180.0 * (PI * (x - 140.0) / 360.0).cos();
                let y2 = 200.0 + 180.0 * (PI * (y - 20.0) / 360.0).cos();

                if i == 0 {
                    context.move_to(x2, y2);
                    bx = x2;
                    by = y2;
                } else {
                    context.line_to(x2, y2);
                }
            }
            context.line_to(bx, by);
        }
    }

    context.stroke();
}
