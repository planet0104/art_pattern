use rgb;
use stdweb::web::CanvasRenderingContext2d;
use stdweb::web::FillRule;

pub fn draw(context: &CanvasRenderingContext2d) {
    context.set_stroke_style_color(&rgb(22, 97, 171));
    context.set_fill_style_color(&rgb(210, 86, 140));

    for _ in 0..50 {
        context.begin_path();
        context.line_to(319.0, 150.0);
        context.line_to(319.0, 199.0);
        context.line_to(160.0, 100.0);
        context.close_path();
        context.fill(FillRule::NonZero);
        context.stroke();
    }
}
