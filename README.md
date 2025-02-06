# SDL2 Particles
![Crates.io Version](https://img.shields.io/crates/v/sdl2_particles)
![docs.rs](https://img.shields.io/docsrs/sdl2_particles)

Rust plug and play library for creating customizable particles with SDL2 context  

## Preview
<img src="preview.gif" width="300" height="300"/>

## Example
Add crate to Cargo.toml and import everything at the beginning of your program:
```rust
use sdl2_particles::*;
```
Create a mutable particle state object and set its size (maximum number of particles):
```rust
let mut particles_state = ParticlesState::init(250);
```
Declare and customize ParticleType:
```rust
let emitting_type = ParticleTypeBuilder::new(16, 16, Duration::from_secs(2))
    .with_color(random_color)
    .with_effect(ParticleEffect::LinearMovement{velocity_x: random_velocity_x,velocity_y: -200.0})
    .with_effect(ParticleEffect::LinearRotation{angular_velocity:60.0})
    .with_effect(ParticleEffect::FadeOut{delay: Duration::from_secs_f32(1.0)})
    .build();
```
Then initialize sdl2 window and context as you would do normally and create a game loop.
Inside you must update and render and spawn your particles 
```rust
loop {
    particles_state.emit(5, &emitting_type, 400.0, 600.0);
    particles_state.update(dt);
    particles_state.render(&mut sdl_canvas);
}
```
More code and previews in examples/ folder