use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

const HOOGTEFACTOR: f32 = 1.0 / 10.0;

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

    let mut camera = raylib::camera::Camera3D::perspective(
        raylib::core::math::Vector3 {
            x: 4.0,
            y: 50.0,
            z: 720.0 * HOOGTEFACTOR,
        },
        raylib::core::math::Vector3 {
            x: 4.2,
            y: 51.5,
            z: 720.0 * HOOGTEFACTOR,
        },
        raylib::core::math::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        45.0,
    );

    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("Dienstregeling")
        .build();

    rl.set_target_fps(60);
    rl.set_camera_mode(camera, CameraMode::CAMERA_FREE);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        {
            let mut d3 = d.begin_mode3D(camera);

            d3.update_camera(&mut camera);

            for rit in &ritjes {
                let hoeveelheidpunten = rit.lijn.len();
                for i in 1..hoeveelheidpunten {
                    let fysiek_punt_a = &rit.lijn[i - 1];
                    let fysiek_punt_b = &rit.lijn[i];

                    let punta = Vector3 {
                        x: fysiek_punt_a.lng,
                        y: fysiek_punt_a.lat,
                        z: fysiek_punt_a.hoogte * HOOGTEFACTOR,
                    };

                    let puntb = Vector3 {
                        x: fysiek_punt_b.lng,
                        y: fysiek_punt_b.lat,
                        z: fysiek_punt_b.hoogte * HOOGTEFACTOR,
                    };

                    d3.draw_line_3D(punta, puntb, Color::YELLOW);
                }
            }
        }

        d.draw_fps(10, 10);
    }

    Ok(())
}
