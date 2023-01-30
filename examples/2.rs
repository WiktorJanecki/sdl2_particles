extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use sdl2_particles as lib;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let timer = sdl_context.timer().unwrap();
    let mut last_frame_time = timer.ticks();
    let mut dt;

    let mut particles_state = lib::ParticlesState::init(5 * 65 * 2); // emitting 5 times per tick 60 times per second(maybe over) particle need to stay alive for 2 seconds

    let window = video_subsystem
        .window("sdl2_particles example: 2", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

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
        dt = Duration::from_millis((timer.ticks() - last_frame_time) as u64);
        last_frame_time = timer.ticks();
        let mut rng = rand::thread_rng();
        let random_velocity_x = rng.gen_range(-50.0..50.0);
        let random_color =
            sdl2::pixels::Color::RGB(rng.gen_range(0..255), 0, rng.gen_range(0..128));

        let emitting_type = lib::ParticleTypeBuilder::new(16, 16, Duration::from_secs(2))
            .with_color(random_color)
            .with_effect(lib::ParticleEffect::LinearMovement{velocity_x: random_velocity_x,velocity_y: -200.0})
            .with_effect(lib::ParticleEffect::LinearRotation{angular_velocity:60.0})
            .with_effect(lib::ParticleEffect::FadeOut{delay: Duration::from_secs_f32(1.0)})
            .build();

        particles_state.emit(5, emitting_type, 400.0, 600.0);
        particles_state.update(dt);

        canvas.set_draw_color(Color::RGB(0, 128, 255));
        canvas.clear();

        particles_state.render(&mut canvas);

        canvas.present();

        std::thread::sleep(Duration::from_secs_f32(1.0 / 60.0));
    }

    Ok(())
}
