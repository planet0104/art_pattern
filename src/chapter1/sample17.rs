use stdweb::web::CanvasRenderingContext2d;
use rgb;
use std::f64::consts::PI;

pub fn draw(context: &CanvasRenderingContext2d){
    context.set_stroke_style_color(&rgb(0, 0, 0));
    let mut i = 1.0;
    let (x, y) = (320.0, 100.0);
    while i<=100.0{
        /*保存translate*/
        context.save();
        context.translate(x, y);
        /*保存scale*/
        context.save();
        context.begin_path();
        context.scale(i/100.0, 1.0);
        context.arc(0.0, 0.0, i, 0.00001, PI*2.0, false);
        context.restore();
        context.stroke();
        context.restore();
        i += 5.0;
    }
}