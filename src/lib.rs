use sdl2::pixels::Color;

impl ParticlesState {
    pub fn init(max_particles: u32) -> ParticlesState {
        let mut pool: Vec<Particle> = vec![];
        for _i in 0..max_particles {
            pool.push(Particle {
                pos_x: 0.0,
                pos_y: 0.0,
                vel_x: 0.0,
                vel_y: 0.0,
                size_x: 0,
                size_y: 0,
                color: Color::WHITE,
                lifetime: 0.0,
                is_alive: false,
                rotation: 0.0,
            })
        }
        return ParticlesState {
            pool,
            emitting_index: 0
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
                particle.lifetime -= dt;
                if particle.lifetime <= 0.0 {
                    particle.is_alive = false
                };
            });
    }
    pub fn emit(
        self: &mut Self,
        emitting_count: u32,
        emitting_type: ParticleType,
        pos_x: f32,
        pos_y: f32,
    ) {
        for _i in 0..emitting_count{
            let particle = self.pool.get_mut(self.emitting_index).unwrap();
            particle.pos_x = pos_x;
            particle.pos_y = pos_y;
            particle.size_x = emitting_type.size_x;
            particle.size_y = emitting_type.size_y;
            particle.color = emitting_type.color;
            particle.is_alive = true;
            particle.lifetime = emitting_type.lifetime;
            emitting_type.effects.iter().for_each(|effect|{
                match effect{
                    ParticleEffect::Moving((x,y)) => {
                        particle.vel_x = *x;
                        particle.vel_y = *y;
                    },
                    ParticleEffect::Rotation(angle) => {
                        particle.rotation = *angle;
                    },
                }
            });
            self.emitting_index+=1;
            if self.emitting_index > self.pool.len() - 1{
                self.emitting_index = 0;
            }
        }
    }
    pub fn render(self: &mut Self, canvas: &mut sdl2::render::WindowCanvas){
        let mut pixel_data = [1];
        let white_surface = sdl2::surface::Surface::from_data(&mut pixel_data, 1, 1, 1, sdl2::pixels::PixelFormatEnum::Index1LSB).unwrap();
        let creator = canvas.texture_creator();
        let mut texture = creator.create_texture_from_surface(white_surface).unwrap();
        self.pool.iter().filter(|particle| particle.is_alive).for_each(|particle|{
            texture.set_color_mod(particle.color.r, particle.color.g, particle.color.b);
            let dst = sdl2::rect::Rect::new(particle.pos_x as i32, particle.pos_y as i32, particle.size_x, particle.size_y);
            let _ = canvas.copy_ex(&texture, None, dst, particle.rotation as f64, None, false, false);
        });
    }
}

pub struct ParticlesState {
    pool: Vec<Particle>,
    emitting_index: usize,
}
struct Particle {
    pos_x: f32,
    pos_y: f32,
    vel_x: f32,
    vel_y: f32,
    size_x: u32,
    size_y: u32,
    rotation: f32,
    color: Color,
    lifetime: f32, // in seconds
    is_alive: bool,
}

pub struct ParticleType {
    effects: Vec<ParticleEffect>,
    lifetime: f32,
    size_x: u32,
    size_y: u32,
    color: Color,
}
pub enum ParticleEffect {
    Rotation(f32),
    Moving((f32, f32)),
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
    pub fn with_effect(mut self: Self, effect: ParticleEffect)  -> Self  {
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
        }
        particles_state.update(Duration::from_secs(5));
        let last = particles_state.pool.get(15).unwrap();
        assert_eq!(last.pos_x, 5.0);
        assert_eq!(last.lifetime, 0.0);
        assert_eq!(last.is_alive, false);
    }

    #[test]
    fn emit_particles(){
        let mut particles_state = crate::ParticlesState::init(16);
        let ptype = crate::ParticleTypeBuilder::new(16,16,Duration::from_secs(4))
            .with_effect(crate::ParticleEffect::Moving((5.0,4.0)))
            .build();
        particles_state.emit(4,ptype,0.0,0.0);
        particles_state.update(Duration::from_secs(2));
        let last = particles_state.pool.get_mut(3).unwrap();
        assert_eq!(last.pos_x, 10.0);
        assert_eq!(last.pos_y, 8.0);
        assert_eq!(last.lifetime, 2.0);
        assert_eq!(last.is_alive, true);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
