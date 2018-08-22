use stdweb::web::CanvasRenderingContext2d;

pub fn draw(context: &CanvasRenderingContext2d) {
    context.stroke_text("例1-4 利用画线语句画一架飞机", 20.0, 20.0, None);

    context.set_stroke_style_color("#000");
    context.set_line_width(1.0);
    context.begin_path();

    let data = [
        420.0, 95.0, 520.0, 60.0, 580.0, 50.0, 540.0, 90.0, 560.0, 115.0, 550.0, 140.0, 500.0,
        145.0, 600.0, 185.0, 530.0, 190.0, 430.0, 170.0, 200.0, 160.0, 500.0, 145.0, 200.0, 160.0,
        90.0, 165.0, 70.0, 155.0, 80.0, 145.0, 150.0, 125.0, 160.0, 115.0, 300.0, 105.0, 310.0,
        95.0, 330.0, 75.0, 360.0, 70.0, 420.0, 95.0, 300.0, 105.0,
    ];

    context.move_to(data[0], data[1]);

    let mut i = 2;
    while i < data.len() {
        context.line_to(data[i], data[i + 1]);
        i += 2;
    }
    context.close_path();
    context.stroke();
}
