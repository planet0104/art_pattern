use stdweb::web::CanvasRenderingContext2d;
use std::f64::consts::PI;

/*半径 */
const R:f64 = 80.0;
/*距离 */
const DISTANCE: f64 = 110.0;

pub fn draw(context: &CanvasRenderingContext2d){
    for px in 1..7{
        for py in 1..7{
            draw_flower(px as f64*DISTANCE, py as f64*DISTANCE, context)
        }
    }
}

/*画一个花*/
fn draw_flower(px:f64, py:f64, context: &CanvasRenderingContext2d){
    context.begin_path();
    let mut a = 0.0;
    while a<2.0*PI*2.0{
        let d = R*(1.0+1.0/5.0*(12.0*a).sin());
        let t = d*(1.0/2.0+1.0/2.0*(4.0*a).sin());
        let x1 = t*a.cos();
        let y1 = -t*a.sin();
        let x = (px+x1)/2.0;
        let y = (py+y1)/2.0;
        if a == 0.0{
            context.move_to(x, y);
        }else{
            context.line_to(x, y);
        }
        a += PI/50.0;
    }
    context.stroke();
}