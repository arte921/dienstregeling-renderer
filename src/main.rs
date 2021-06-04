use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Punt {
    lat: f32,
    lng: f32,
    hoogte: f32,
}

#[derive(Serialize, Deserialize)]
struct Rit {
    vertrektijd: u16,
    aankomsttijd: u16,
    lijn: Vec<Punt>,
}

fn main() -> std::io::Result<()> {
    let mut ritjes_json_file = File::open("opslag/alleritjes.json")?;
    let mut ritjes_json = String::new();
    ritjes_json_file.read_to_string(&mut ritjes_json)?;

    let ritjes: Vec<Rit> = serde_json::from_str(&ritjes_json)?;

    println!("{}", ritjes[0].lijn[0].hoogte);

    let camera = raylib::camera::Camera3D::perspective(
        raylib::core::math::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        raylib::core::math::Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        raylib::core::math::Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        45.0,
    );

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Dienstregeling")
        .build();
    
    rl.set_target_fps(60);
    
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::BLACK);

        let mut d3 = d.begin_mode3D(camera);

        d3.draw_line_3D(
            Vector3 {
                x: 2.0,
                y: 0.0,
                z: 0.0
            },
            Vector3 {
                x: 2.0,
                y: 10.0, 
                z: 0.0
            },
            Color::YELLOW
        );
    }

    Ok(())
}
