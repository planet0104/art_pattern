//#![no_main]
extern crate piston_window;
mod canvas;

use piston_window::*;

fn p(s:&String){
    println!("{}", s);
}

fn main(){
    p(&"hello".to_string());
    let mut window: PistonWindow = 
            WindowSettings::new("Rust艺术图案程序设计", [640, 480])
            .exit_on_esc(true).build().unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.0; 4], g);
            // let transform = c.transform.trans(100.0, 100.0)
            //                         .rot_rad(0.5);
            // rectangle([1.0, 0.0, 0.0, 1.0], // red
            //         [0.0, 0.0, 100.0, 100.0],
            //         transform, g);
            polygon([1.0; 4],
                    &vec![[10.0, 10.0],[10.0, 10.0],[10.0, 10.0]],
                    c.transform,
                    g);
        });
    }
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern fn WinMain() -> i32 {
    main();
    0
}