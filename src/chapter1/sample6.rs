use std::f64::consts::PI;
use stdweb::web::CanvasRenderingContext2d;

pub fn draw(context: &CanvasRenderingContext2d) {
    let t = 3.0;
    let d = 100.0;

    let mut a = 0.0;
    context.begin_path();
    while a < PI * 2.0 {
        let b = d + d / 5.0 * (3.0 * t * a).sin();
        let c = b * (1.0 / 2.0 + 1.0 / 2.0 * (t * a).sin());

        let x = 320.0 + c * a.cos() * 7.0 / 5.0;
        let y = 100.0 - c * a.sin();

        if a == 0.0 {
            context.move_to(x, y);
        } else {
            context.line_to(x, y);
        }

        a += PI / (20.0 * t);
    }
    context.close_path();
    context.stroke();

    context.stroke_text("例1-6 花形图案", 10.0, 260.0, None);
}
