mod drawing;
mod ml_core;
mod models;

use drawing::drawing::{Color, Drawing, Location2, Location4};
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::{font::Font, source::SystemSource};
use minifb::{MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};
use ml_core::http_tools;
use nfd::Response;
use parking_lot::Mutex;
use raqote::{
    DrawOptions, DrawTarget, PathBuilder, Point, SolidSource, Source, StrokeStyle, Transform,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::{io::Error, thread, time};
use webbrowser;

const WIDTH: usize = 854;
const HEIGHT: usize = 480;

struct Release {
    name: String,
    tag_name: String,
    download_url: String,
    installed: bool,
    stable: bool,
}

struct State {
    version_loading_error: bool
}

fn main() {
    let mut options = WindowOptions::default();
    options.resize = false;

    let versions = Arc::new(Mutex::new(Vec::new()));
    let state = Arc::new(Mutex::new(State {
        version_loading_error: false
    }));

    let mut window = Window::new("MindLaunch 1.0", WIDTH, HEIGHT, options).unwrap();
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::Monospace], &Properties::new())
        .unwrap()
        .load()
        .unwrap();

    // gui vars
    let mut frames: f32 = 0.;
    let mut offset: f32 = 0.0;
    let mut coldown = false;
    fn load_versions(versions_clone: Arc<Mutex<Vec<Release>>>, state_clone: Arc<Mutex<State>>) {
        versions_clone.lock().clear();
        thread::spawn(move || -> Result<(), Error> {
            //thread::sleep_ms(35000);
            // get stable releases
            let data: String =
                http_tools::http_request("https://api.github.com/repos/Anuken/Mindustry/releases")?;
            let data_json: Result<Vec<models::release::Root>, serde_json::error::Error> = serde_json::from_str(&data);
            match data_json {
                Ok(json) => {
                    for version in json.iter() {
                        versions_clone.lock().push(Release {
                            download_url: version.assets[0].clone().browser_download_url.unwrap(),
                            installed: false,
                            name: version
                                .clone()
                                .name
                                .unwrap_or("Unknown version string".to_string()),
                            stable: true,
                            tag_name: version.clone().tag_name.unwrap_or("Unknown".to_string()),
                        });
                    }
                }
                Err(_) => {
                    state_clone.lock().version_loading_error = true;
                }
            }
            // bleeding edge
            let data: String = http_tools::http_request(
                "https://api.github.com/repos/Anuken/MindustryBuilds/releases",
            )?;
            let data_json: Vec<models::release::Root> = serde_json::from_str(&data).unwrap();
            for version in data_json.iter() {
                versions_clone.lock().push(Release {
                    download_url: version.assets[0].clone().browser_download_url.unwrap(),
                    installed: false,
                    name: version
                        .clone()
                        .name
                        .unwrap_or("Bleeding edge build".to_string()),
                    stable: false,
                    tag_name: version.clone().tag_name.unwrap_or("Unknown".to_string()),
                });
            }
            Ok(())
        });
    }

    let mut settings = config::Config::default();
    settings
        // Add in `./Settings.toml`
        .merge(config::File::with_name("Settings")).unwrap()
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .merge(config::Environment::with_prefix("APP")).unwrap();

    let mut settings_hash = settings.try_into::<HashMap<String, String>>().unwrap();

    load_versions(versions.clone(), state.clone());
    let size = window.get_size();
    // programm processsing
    while window.is_open() {
        let current = time::Instant::now();
        frames += 1.;
        // if resize
        let mut drawing = Drawing::new(DrawTarget::new(size.0 as i32, size.1 as i32), &font);

        // event processing
        window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
            if window.get_mouse_down(MouseButton::Left) && !coldown {
                drawing.process_mouse(mouse.0, mouse.1);
                println!("elapsed: {}", drawing.last_click.elapsed().as_secs_f32());
                coldown = true;
            }

            if (frames % 3.0) <= 0.0 {
                coldown = false;
            }
        });

        // Drawing listbox square
        drawing.draw_square(
            Location4::new(5., 0., 300., 480.),
            Color::new(255, 50, 50, 50),
        );

        // Drawing play buttons
        {
            let versions_lock = versions.lock();
            drawing.draw_text(
                "PLAY:",
                Location2::new(320.0, 15.0),
                Color::new(255, 255, 255, 255),
                15.0,
            );
            if versions_lock.len() > 0 {
                drawing.draw_button(
                    &format!("last version {}", versions_lock[0].tag_name),
                    Location4::new(320.0, 20.0, 200.0, 32.0),
                    Color::new(255, 100, 120, 100),
                );
                {
                    for version in versions_lock.iter() {
                        if !version.stable {
                            drawing.draw_button(
                                &format!("last bleeding edge {}", version.tag_name),
                                Location4::new(320.0, 55.0, 200.0, 32.0),
                                Color::new(255, 120, 100, 100),
                            );
                            break;
                        }
                    }
                    drop(versions_lock);
                }
            }
        }
        // Utils
        if drawing.draw_button("Reload", Location4::new(310.0, 450.0, 50.0, 24.0), Color::new(255, 100, 110, 100)) {
            load_versions(versions.clone(), state.clone());
        }
        drawing.draw_text("Click here to change Mindustry installation location: ", Location2::new(365.0, 440.0), Color::new(255, 255, 255, 255), 15.0);
        if drawing.draw_button(&format!("{}", settings_hash.get("path").unwrap()), Location4::new(365.0, 450.0, 460.0, 24.0), Color::new(255, 100, 100, 100)) {
            let result = nfd::open_pick_folder(None).unwrap_or_else(|e| {
                panic!(e);
            });
    
            match result {
                Response::Okay(file_path) => { 
                    println!("File path = {:?}", file_path);
                    settings_hash.insert("path".to_string(), file_path);
            },
                Response::Cancel => println!("User canceled"),
                _ => (),
            }
        }
        // Drawing options
        

        // offset of scrllbox
        let mut idx: f32 = -27.0;

        if versions.lock().len() <= 0 {
            if state.lock().version_loading_error {
                drawing.draw_text(
                    "Version loading ERROR please try later",
                    Location2::new(15.0, 240.0),
                    Color::new(255, 255, 0, 0),
                    14.0,
                );

                if drawing.draw_button("Try again", Location4::new(25.0, 260.0, 60.0, 24.0), Color::new(255, 100, 120, 100)) {
                    load_versions(versions.clone(), state.clone());
                }

            } else {
                drawing.draw_text(
                    "Loading versions...",
                    Location2::new(75.0, 240.0),
                    Color::new(255, 255, 255, 255),
                    15.0,
                );
            }
        } else {
            for version in versions.lock().iter() {
                let mut release_color = Color::new(255, 110, 110, 110); // if stable
                if !version.stable {
                    release_color = Color::new(255, 180, 110, 110); // bleeding edge
                }
                idx += 27.0; // offset of list
                             // version badge
                let baseloc = Location4::new(5., idx + offset, 200.0, 25.0);
                drawing.draw_button(
                    &format!("{} - {}", version.name, version.tag_name),
                    baseloc,
                    release_color,
                );
                //install button
                let baseloc_install = Location4::new(baseloc.x + 200.0, baseloc.y, 50.0, 25.0);
                drawing.draw_button("INSTALL", baseloc_install, Color::new(255, 110, 110, 200));
                //unistall
                let baseloc_install = Location4::new(baseloc.x + 250.0, baseloc.y, 50.0, 25.0);
                drawing.draw_button("DELETE", baseloc_install, Color::new(255, 200, 110, 200));
            }
        }

        // draw scrollbar
        drawing.draw_square(
            Location4::new(
                300.0,
                ((offset - offset * 2.0) / 6.0) % 480.0,
                4.0,
                idx / 8.0,
            ),
            Color::new(255, 110, 110, 110),
        );

        // lock ~60 FPS
        std::thread::sleep(std::time::Duration::from_millis(15));

        let frame = format!(
            "render time: {} secs frames rendered: {} FPS: ~{} drawcalls: {}",
            current.elapsed().as_secs_f32(),
            frames,
            1.0 / current.elapsed().as_secs_f32(),
            drawing.count,
        );

        drawing.draw_text(
            &frame,
            Location2::new(20.0, 20.0),
            Color::new(255, 0, 0, 255),
            12.0,
        );

        window.get_scroll_wheel().map(|scroll| {
            offset += scroll.1 * 4.0;
        });

        let size = window.get_size();
        window
            .update_with_buffer(drawing.dt.get_data(), size.0, size.1)
            .unwrap();
    }
}
