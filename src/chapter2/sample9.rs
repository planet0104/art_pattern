use {get_param, show_params};
use stdweb::web::CanvasRenderingContext2d;

const SHAPE1: [(f64, f64); 7] = [
    (79.0, 14.0),
    (109.0, -17.0),
    (140.0, 14.0),
    (124.0, 14.0),
    (124.0, 70.0),
    (95.0, 70.0),
    (95.0, 14.0),
];
const SHAPE2: [(f64, f64); 7] = [
    (59.0, 51.0),
    (89.0, 20.0),
    (120.0, 51.0),
    (104.0, 51.0),
    (104.0, 107.0),
    (75.0, 107.0),
    (75.0, 51.0),
];
const SHAPE3: [(f64, f64); 7] = [
    (39.0, 88.0),
    (69.0, 57.0),
    (100.0, 88.0),
    (84.0, 88.0),
    (84.0, 144.0),
    (55.0, 144.0),
    (55.0, 88.0),
];

pub fn draw(context: &CanvasRenderingContext2d) {
    show_params();
    context.set_stroke_style_color("#000");

    /*读取参数*/
    let a = get_param(0).parse::<f64>().unwrap_or(1.0);
    let b = get_param(1).parse::<f64>().unwrap_or(0.0);
    let c = get_param(2).parse::<f64>().unwrap_or(0.0);
    let d = get_param(3).parse::<f64>().unwrap_or(1.0);

    /*显示输入参数 */
    context.stroke_text(&format!("[ {} , {} ]", a, b), 50.0, 50.0, None);
    context.stroke_text(&format!("[ {} , {} ]", c, d), 50.0, 70.0, None);
    context.stroke_text("四个参数:", 50.0, 90.0, None);
    context.stroke_text("原型图形    [1, 0, 0, 1]", 50.0, 110.0, None);
    context.stroke_text("纵向放大2倍 [1, 0, 0, 2]", 50.0, 130.0, None);
    context.stroke_text("对称变换    [-2, 0, 0, 2]", 50.0, 150.0, None);
    context.stroke_text("错切变换    [1, 0.5, 0, 1]", 50.0, 170.0, None);

    /*矩阵变换 */
    let transform = |x: f64, y: f64| -> (f64, f64) { (a * x + c * y, b * x + d * y) };

    /*画图 */
    draw_shape(
        context,
        &SHAPE1
            .iter()
            .map(|point| transform(point.0, point.1))
            .collect(),
    );
    draw_shape(
        context,
        &SHAPE2
            .iter()
            .map(|point| transform(point.0, point.1))
            .collect(),
    );
    draw_shape(
        context,
        &SHAPE3
            .iter()
            .map(|point| transform(point.0, point.1))
            .collect(),
    );
}

fn draw_shape(context: &CanvasRenderingContext2d, points: &Vec<(f64, f64)>) {
    context.save();
    context.translate(300.0, 300.0);
    context.begin_path();
    context.move_to(points[0].0, points[0].1);
    for i in 1..points.len() {
        context.line_to(points[i].0, points[i].1);
    }
    context.line_to(points[0].0, points[0].1);
    context.stroke();
    context.restore();
}
