use sdl2::pixels::Color;

impl ParticlesState {
    pub fn init(max_particles: u32) -> ParticlesState {
        let mut pool: Vec<Particle> = vec![];
        for _i in 0..max_particles {
            pool.push(Particle::new())
        }
        return ParticlesState {
            pool,
            emitting_index: 0,
        };
    }
    pub fn update(self: &mut Self, delta_time: std::time::Duration) {
        let dt = delta_time.as_secs_f32();
        self.pool
            .iter_mut()
            .filter(|particle| particle.is_alive)
            .for_each(|particle| {
                particle.pos_x += particle.vel_x * dt;
                particle.pos_y += particle.vel_y * dt;
                particle.rotation = (particle.rotation + particle.vel_angular * dt) % 360.0;
                particle.lifetime -= dt;
                if particle.lifetime <= 0.0 {
                    particle.is_alive = false
                };
                if particle.lifetime < particle.fade.1 {
                    let how_much_opacity_to_lower = particle.fade.2 as f32 * dt;
                    particle.alpha -= how_much_opacity_to_lower.max(0.0);
                }
            });
    }
    pub fn emit(
        self: &mut Self,
        emitting_count: u32,
        emitting_type: &ParticleType,
        pos_x: f32,
        pos_y: f32,
    ) {
        for _i in 0..emitting_count {
            let particle = self.pool.get_mut(self.emitting_index).unwrap();
            *particle = Particle::new(); // reset particle
            particle.pos_x = pos_x;
            particle.pos_y = pos_y;
            particle.size_x = emitting_type.size_x;
            particle.size_y = emitting_type.size_y;
            particle.color = emitting_type.color;
            particle.is_alive = true;
            particle.lifetime = emitting_type.lifetime;
            emitting_type
                .effects
                .iter()
                .for_each(|effect| match effect {
                    ParticleEffect::LinearMovement { velocity_x, velocity_y } => {
                        particle.vel_x = *velocity_x;
                        particle.vel_y = *velocity_y;
                    }
                    ParticleEffect::ConstantRotation{ angle } => {
                        particle.rotation = *angle;
                    }
                    ParticleEffect::LinearRotation{angular_velocity} => {
                        particle.vel_angular = *angular_velocity;
                    }
                    ParticleEffect::FadeOut{delay} => {
                        let when_should_fade = particle.lifetime - delay.as_secs_f32();
                        let how_much_time_will_be_fading = when_should_fade;
                        let speed_of_fading_per_sec = 256.0 / how_much_time_will_be_fading;
                        particle.fade = (true, when_should_fade, speed_of_fading_per_sec);
                    }
                });
            self.emitting_index += 1;
            if self.emitting_index > self.pool.len() - 1 {
                self.emitting_index = 0;
            }
        }
    }
    pub fn render(self: &mut Self, canvas: &mut sdl2::render::WindowCanvas){
        self.render_with_offset(0, 0, canvas);
    }
    pub fn render_with_offset(self: &mut Self, offset_x: i32, offset_y: i32, canvas: &mut sdl2::render::WindowCanvas) {
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        let mut pixel_data = [1];
        let white_surface = sdl2::surface::Surface::from_data(
            &mut pixel_data,
            1,
            1,
            1,
            sdl2::pixels::PixelFormatEnum::Index1LSB,
        )
        .unwrap();
        let creator = canvas.texture_creator();
        let mut texture = creator.create_texture_from_surface(white_surface).unwrap();
        texture.set_blend_mode(sdl2::render::BlendMode::Blend);
        self.pool
            .iter()
            .filter(|particle| particle.is_alive)
            .for_each(|particle| {
                texture.set_color_mod(particle.color.r, particle.color.g, particle.color.b);
                texture.set_alpha_mod(particle.alpha as u8);
                let dst = sdl2::rect::Rect::new(
                    particle.pos_x as i32 + offset_x,
                    particle.pos_y as i32 + offset_y,
                    particle.size_x,
                    particle.size_y,
                );
                let _ = canvas.copy_ex(
                    &texture,
                    None,
                    dst,
                    particle.rotation as f64,
                    None,
                    false,
                    false,
                );
            });
        #[cfg(feature="use-unsafe_textures")]
        unsafe{
            texture.destroy();
        }
    }
}

pub struct ParticlesState {
    pool: Vec<Particle>,
    emitting_index: usize,
}

#[derive(Clone)]
struct Particle {
    pos_x: f32,
    pos_y: f32,
    vel_x: f32,
    vel_y: f32,
    vel_angular: f32,
    size_x: u32,
    size_y: u32,
    rotation: f32,
    color: Color,
    alpha: f32,             //0 - 255
    fade: (bool, f32, f32), // should fade, on what lifetime value should start fading, how much opacity per sec
    lifetime: f32,          // in seconds
    is_alive: bool,
}
impl Particle {
    fn new() -> Self {
        Self {
            pos_x: 0.0,
            pos_y: 0.0,
            vel_x: 0.0,
            vel_y: 0.0,
            vel_angular: 0.0,
            size_x: 0,
            size_y: 0,
            rotation: 0.0,
            color: Color::WHITE,
            fade: (false, 0.0, 0.0),
            lifetime: 0.0,
            is_alive: false,
            alpha: 255.0,
        }
    }
}

#[derive(Clone)]
pub struct ParticleType {
    effects: Vec<ParticleEffect>,
    lifetime: f32,
    size_x: u32,
    size_y: u32,
    color: Color,
}

#[derive(Clone, Copy)]
pub enum ParticleEffect {
    ConstantRotation{angle:f32},
    LinearMovement{velocity_x:f32, velocity_y:f32},
    LinearRotation{angular_velocity: f32},
    FadeOut{delay: std::time::Duration},
}

pub struct ParticleTypeBuilder {
    effects: Vec<ParticleEffect>,
    lifetime: std::time::Duration,
    size_x: u32,
    size_y: u32,
    color: Color,
}

impl ParticleTypeBuilder {
    pub fn new(size_x: u32, size_y: u32, lifetime: std::time::Duration) -> Self {
        Self {
            effects: vec![],
            lifetime,
            size_x,
            size_y,
            color: Color::WHITE,
        }
    }
    pub fn with_color(mut self: Self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn with_effect(mut self: Self, effect: ParticleEffect) -> Self {
        self.effects.push(effect);
        self
    }
    pub fn build(self: Self) -> ParticleType {
        ParticleType {
            effects: self.effects,
            lifetime: self.lifetime.as_secs_f32(),
            size_x: self.size_x,
            size_y: self.size_y,
            color: self.color,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    #[test]
    fn init_particles() {
        let particles_state = crate::ParticlesState::init(16);
        let last = particles_state.pool.get(15).unwrap();
        assert_eq!(last.color, sdl2::pixels::Color::WHITE);
    }

    #[test]
    fn update_particles() {
        let mut particles_state = crate::ParticlesState::init(16);
        {
            let mut last = particles_state.pool.get_mut(15).unwrap();
            last.vel_x = 1.0;
            last.lifetime = 5.0;
            last.is_alive = true;
            last.vel_angular = 180.0;
        }
        particles_state.update(Duration::from_secs(5));
        let last = particles_state.pool.get(15).unwrap();
        assert_eq!(last.pos_x, 5.0);
        assert_eq!(last.lifetime, 0.0);
        assert_eq!(last.is_alive, false);
        assert_eq!(last.rotation, 180.0);
    }

    #[test]
    fn emit_particles() {
        let mut particles_state = crate::ParticlesState::init(16);
        let ptype = crate::ParticleTypeBuilder::new(16, 16, Duration::from_secs(4))
            .with_effect(crate::ParticleEffect::LinearMovement{velocity_x:5.0, velocity_y:4.0})
            .build();
        particles_state.emit(4, &ptype, 0.0, 0.0);
        particles_state.update(Duration::from_secs(2));
        let last = particles_state.pool.get_mut(3).unwrap();
        assert_eq!(last.pos_x, 10.0);
        assert_eq!(last.pos_y, 8.0);
        assert_eq!(last.lifetime, 2.0);
        assert_eq!(last.is_alive, true);
    }

    #[test]
    fn particles_fading() {
        let mut particles_state = crate::ParticlesState::init(4);
        let ptype = crate::ParticleTypeBuilder::new(16, 16, Duration::from_secs(4))
            .with_effect(crate::ParticleEffect::FadeOut{delay: Duration::from_secs_f32(2.0)})
            .build();
        particles_state.emit(4, &ptype, 0.0, 0.0);
        {
            let last = particles_state.pool.get_mut(3).unwrap();
            assert_eq!(last.alpha, 255.0);
        }
        particles_state.update(Duration::from_secs(1));
        particles_state.update(Duration::from_secs(1));
        particles_state.update(Duration::from_secs(1));
        let last = particles_state.pool.get_mut(3).unwrap();
        assert_eq!(last.alpha, 127.0);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
