use std::f64::consts::PI;
use stdweb::web::CanvasRenderingContext2d;

pub fn draw(context: &CanvasRenderingContext2d) {
    context.set_stroke_style_color("#000");

    /*正方形边长 */
    let l = 180.0;
    let m = 2.0 * l / 3.0f64.sqrt();
    /*六瓣花形的半径 */
    let d = l / 4.0 * 1.3;

    context.begin_path();

    let mut aa = 0.0;
    /*旋转循环，步长π/3，将六个变换后的三角形图形填入正六边形内 */
    while aa <= 5.0 * PI / 3.0 {
        let mut px = l / 4.0;
        /*画正方形内图案的循环语句 */
        while px <= l / 4.0 + l / 2.0 {
            let mut py = l / 4.0;
            while py <= l / 4.0 + l / 2.0 {
                /*画六瓣花型 */
                let mut a = 0.0;
                while a <= 2.0 * PI + PI / 60.0 {
                    /*计算六瓣花型的坐标 */
                    let x0 = d * (1.0 / 2.0 + 1.0 / 2.0 * (6.0 * a).sin()) * a.cos() + px;
                    let y0 = d * (1.0 / 2.0 + 1.0 / 2.0 * (6.0 * a).sin()) * a.sin() + py;

                    /*进行三角形变换 */
                    let x2 = x0 * y0 * m / (l * l) + (l - y0) * m / l / 2.0 - m / 2.0;

                    let x = x2 * aa.cos() - y0 * aa.sin() + 320.0;
                    let y = x2 * aa.sin() + y0 * aa.cos() + 220.0;

                    if a == 0.0 {
                        context.move_to(x, y);
                    } else {
                        context.line_to(x, y);
                    }
                    a += PI / 48.0;
                }
                py += l / 2.0;
            }
            px += l / 2.0;
        }
        aa += PI / 3.0;
    }

    context.stroke();
}
