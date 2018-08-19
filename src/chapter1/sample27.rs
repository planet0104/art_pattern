use stdweb::web::CanvasRenderingContext2d;
use rgb;
use std::f64::consts::PI;

pub fn draw(context: &CanvasRenderingContext2d){
    context.set_fill_style_color(&rgb(255, 0, 0));
    context.save();
    context.translate(200.0, 0.0);
    let mut x = 0.0;
    let scale = 100.0;
    while x < 2.0*PI{
        let y = x.sin();
        context.fill_text("*", y*scale, x*scale, None);
        x+=0.08;
    }
    context.restore();
}