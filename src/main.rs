#![no_main]
extern crate piston_window;

use piston_window::*;

//启动
fn start(){
    let mut window: PistonWindow = 
            WindowSettings::new("Rust艺术图案程序设计", [640, 480])
            .exit_on_esc(true).build().unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            let transform = c.transform.trans(100.0, 100.0)
                                    .rot_rad(0.5);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                    [0.0, 0.0, 100.0, 100.0],
                    transform, g);
        });
    }
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern fn WinMain() -> i32 {
    start();
    0
}