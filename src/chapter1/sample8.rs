use stdweb::web::CanvasRenderingContext2d;
use {rand_int, rgb};

pub fn draw(context: &CanvasRenderingContext2d) {
    context.begin_path();
    let mut p = 7.0;
    while p <= 319.0 {
        context.set_fill_style_color(&rgb(rand_int(0, 255), rand_int(0, 255), rand_int(0, 255)));
        let (left, top) = (0.25 * p, 0.1 * p);
        let (right, bottom) = (p, 0.3 * p);
        context.fill_rect(left, top, right - left, bottom - top);
        p += 32.0;
    }
    context.close_path();
    context.stroke();

    context.stroke_text(
        "例1-8 由小到大变化的着色矩形",
        10.0,
        260.0,
        None,
    );
}
