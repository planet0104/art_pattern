use stdweb::web::CanvasRenderingContext2d;
use rgb;
use stdweb::web::FillRule;

pub fn draw(context: &CanvasRenderingContext2d){
    
    context.set_fill_style_color(&rgb(210, 86, 140));

    for _ in 0..50{

        //上面
        context.begin_path();
        context.set_fill_style_color(&rgb(255, 87, 87));
        context.move_to(100.0, 80.0);
        context.line_to(180.0, 80.0);
        context.line_to(210.0, 50.0);
        context.line_to(130.0, 50.0);
        context.line_to(100.0, 80.0);
        context.fill(FillRule::NonZero);

        //右面
        context.begin_path();
        context.set_fill_style_color(&rgb(255, 255, 87));
        context.move_to(180.0, 150.0);
        context.line_to(210.0, 120.0);
        context.line_to(210.0, 50.0);
        context.line_to(180.0, 80.0);
        context.line_to(180.0, 150.0);
        context.line_to(200.0, 100.0);
        context.fill(FillRule::NonZero);

        //字母A
        context.begin_path();
        context.set_stroke_style_color(&rgb(28, 41, 56));
        context.move_to(110.0, 140.0);
        context.line_to(135.0, 90.0);
        context.line_to(145.0, 90.0);
        context.line_to(170.0, 140.0);
        context.line_to(160.0, 140.0);
        context.line_to(155.0, 130.0);
        context.line_to(125.0, 130.0);
        context.line_to(120.0, 140.0);
        context.line_to(110.0, 140.0);
        context.move_to(130.0, 120.0);
        context.line_to(150.0, 120.0);
        context.line_to(140.0, 100.0);
        context.line_to(130.0, 120.0);
        context.stroke();
    }
}