use std::f64::consts::PI;
use stdweb::web::CanvasRenderingContext2d;

pub fn draw(context: &CanvasRenderingContext2d) {
    context.set_stroke_style_color("#000");
    context.begin_path();

    /* 每循环一次画一条正弦曲线, 同时y方向上向下平移4点 */
    for py in (25..=175).step_by(3).map(|py| py as f64) {
        let mut al = 0.0;

        while al <= 4.0 * PI {
            let x = 360.0 / (4.0 * PI) * al + 140.0;
            let p = 10.0 * (2.0 * al).sin();
            let y = py - p * (al - 2.0 * PI / 150.0 * (py - 25.0)).sin();
            if al == 0.0 {
                context.move_to(x, y);
            } else {
                context.line_to(x, y);
            }
            al += PI / 40.0;
        }
    }
    context.stroke();
}
