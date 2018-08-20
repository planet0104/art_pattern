use stdweb::web::CanvasRenderingContext2d;

/*顶点 */
const VERTEXS:[(f64, f64); 3] = [(100.0, 100.0), (220.0, 100.0), (160.0, 50.0)];

pub fn draw(context: &CanvasRenderingContext2d){
    
    /*画原三角形 */
    draw_triangle(&VERTEXS, context, "#000");

    let (scale_x, scale_y, center_x, center_y) = (0.5, 0.5, 160.0, 75.0);

    /*画缩放后的三角形 */
    let scale = |x: f64, y: f64| -> (f64, f64){
        (x*scale_x+center_x*(1.0-scale_x), y*scale_y+center_y*(1.0-scale_y)) 
    };

    draw_triangle(&[
        scale(VERTEXS[0].0, VERTEXS[0].1),
        scale(VERTEXS[1].0, VERTEXS[1].1),
        scale(VERTEXS[2].0, VERTEXS[2].1)
    ], context, "#00f");
}

/*画三角形 */
fn draw_triangle(points: &[(f64, f64); 3], context:&CanvasRenderingContext2d, color:&str){
    context.set_stroke_style_color(color);
    context.begin_path();
    context.move_to(points[0].0, points[0].1);
    context.line_to(points[1].0, points[1].1);
    context.line_to(points[2].0, points[2].1);
    context.line_to(points[0].0, points[0].1);
    context.stroke();
}