use std::f64::consts::PI;
use stdweb::web::CanvasRenderingContext2d;

pub fn draw(context: &CanvasRenderingContext2d) {
    context.set_stroke_style_color("#000");

    let rotate = |x: f64, y: f64, a: f64| -> (f64, f64) {
        (x * a.cos() + y * a.sin(), y * a.cos() - x * a.sin())
    };

    let t = (170.0f64 * 170.0 * 2.0).sqrt();
    context.begin_path();
    for x in (-170..170).step_by(20) {
        for y in (-170..170).step_by(20) {
            let (x, y) = (x as f64, y as f64);
            let a = (x * x + y * y).sqrt() / t * PI / 2.0 + PI / 4.0;
            let mut al = 0.0;
            while al <= 2.0 * PI {
                let (x1, y1) = (10.0 * al.cos(), 10.0 * al.sin());
                let (x2, y2) = rotate(x1, y1, a);
                let (x, y) = (x + 320.0 + x2, y + 200.0 - y2);
                if al == 0.0 {
                    context.move_to(x, y);
                } else {
                    context.line_to(x, y);
                }
                al += PI / 2.0;
            }
        }
    }
    context.stroke();
}
