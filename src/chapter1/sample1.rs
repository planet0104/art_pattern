use stdweb::web::CanvasRenderingContext2d;

pub fn draw(context: &CanvasRenderingContext2d) {
    let mut x = 0.0;
    while x < 2.0 * ::std::f64::consts::PI * 2.0 {
        let ysin = 1.0 - x.sin();
        let ycos = 1.0 - x.cos();
        context.set_fill_style_color("#f00");
        context.fill_rect(50.0 * x, 99.0 * ysin, 1.0, 1.0);
        context.set_fill_style_color("#000");
        context.fill_rect(50.0 * x, 99.0 * ycos, 1.0, 1.0);
        x += 0.01;
    }

    context.stroke_text(
        "例1-1 画一条正弦曲线和余弦曲线",
        10.0,
        260.0,
        None,
    );
}
