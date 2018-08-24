use std::f64::consts::PI;
use stdweb::web::CanvasRenderingContext2d;

pub fn draw(context: &CanvasRenderingContext2d) {
    context.set_stroke_style_color("#000");
    context.begin_path();

    /* 每循环一次画一条正弦曲线, 同时y方向上向下平移4点 */
    for py in (25..=175).step_by(4).map(|py| py as f64) {
        let mut al = 0.0;
        /* x轴取角度值 以al为变量画正弦曲线 */
        while al <= 4.0 * PI {
            /* 计算x坐标值 */
            let x = 360.0 / (4.0 * PI) * al + 140.0;
            /* y轴取 asin(al) */
            let y = py - 10.0 * (al - 2.0 * PI / 300.0 * (py - 25.0)).sin();
            if al == 0.0 {
                context.move_to(x, y);
            } else {
                context.line_to(x, y);
            }
            al += PI / 20.0;
        }
    }
    context.stroke();
}
