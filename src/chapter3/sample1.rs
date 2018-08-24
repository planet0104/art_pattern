use std::f64::consts::PI;
use stdweb::web::CanvasRenderingContext2d;

pub fn draw(context: &CanvasRenderingContext2d) {
    context.set_stroke_style_color("#000");
    context.begin_path();
    /*在半径80的圆周上取等间隔点为圆心 */
    let l = 80.0;
    let mut al = 0.0;
    while al <= 2.0 * PI {
        /*计算圆心坐标 */
        let (x1, y1) = (l * al.cos(), l * al.sin());
        /*将圆移动到屏幕中央 */
        let (x2, y2) = (x1 + 391.5, y1 + 300.0);
        /*圆的半径为50 */
        context.arc(x2, y2, 50.0, 0.0, PI * 2.0, true);
        al += PI / 24.0;
    }
    context.stroke();
}
