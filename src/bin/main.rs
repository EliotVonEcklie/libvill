use sdl2::{event::Event, video::Window, keyboard::Keycode, pixels::Color, render::Canvas};
use std::time::Duration;
use bevy::prelude::*;
use libvill::{VillPlugin, util::ModelComponent};

fn main_runner(mut app: App) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Render", 1280, 720)
        .allow_highdpi()
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string()).unwrap();
    
    let canvas = window.into_canvas().build().map_err(|e| e.to_string()).unwrap();

    app.insert_non_send_resource(canvas);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let mut canvas = app.world.non_send_resource_mut::<Canvas<Window>>();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        app.update();
                
        let mut canvas = app.world.non_send_resource_mut::<Canvas<Window>>();
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn().insert(ModelComponent::new("test"));
}

fn main() -> Result<(), String> {
    App::new()
        .add_plugin(VillPlugin)
        .add_startup_system(setup)
        .set_runner(main_runner)
        .run();

    Ok(())
}
