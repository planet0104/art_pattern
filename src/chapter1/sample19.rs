use std::f64::consts::PI;
use stdweb::web::CanvasRenderingContext2d;
use stdweb::web::FillRule;
use {rand_int, random, rgb};

pub fn draw(context: &CanvasRenderingContext2d) {
    context.set_stroke_style_color(&rgb(237, 90, 101));
    for _ in 0..50 {
        context.begin_path();
        context.arc(
            random() * 400.0,
            random() * 400.0,
            random() * 40.0,
            0.0,
            PI * 2.0,
            false,
        );
        context.close_path();
        context.set_fill_style_color(&rgb(rand_int(0, 255), rand_int(0, 255), rand_int(0, 255)));
        context.fill(FillRule::NonZero);
        context.stroke();
    }
}
