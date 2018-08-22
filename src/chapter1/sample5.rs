use stdweb::web::CanvasRenderingContext2d;

pub fn draw(context: &CanvasRenderingContext2d) {
    context.set_stroke_style_color("#000");
    context.set_line_width(1.0);
    context.begin_path();

    let x_max = 540.0;
    let y_max = 190.0;
    let x_min = 100.0;
    let y_min = 10.0;

    let s = 20.0;
    let xs = (x_max - x_min) / s;
    let ys = (y_max - y_min) / s;

    let mut i = 0.0;
    while i < s {
        context.move_to(x_min + xs * i, y_max);
        context.line_to(x_max, y_max - ys * i);

        context.move_to(x_max - xs * i, y_min);
        context.line_to(x_max, y_max - ys * i);

        context.move_to(x_max - xs * i, y_min);
        context.line_to(x_min, y_min + ys * i);

        context.move_to(x_min, y_min + ys * i);
        context.line_to(x_min + xs * i, y_max);

        i += 1.0;
    }
    context.close_path();
    context.stroke();

    context.stroke_text("例1-5 几何图案", 20.0, 250.0, None);
}
