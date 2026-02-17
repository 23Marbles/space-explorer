use godot::{classes::{IRigidBody2D, RigidBody2D}, meta::PackedArrayElement, obj::Base, prelude::*};

#[derive(GodotClass)]
#[class(base=RigidBody2D, init)]
struct GravObject {
    #[export]
    #[init(val = 10.)]
    threshhold_range: f32,
    #[export]
    hold_time: f64,
    #[export]
    #[init(val = 40000.)]
    max_turn_speed: f32,
    #[export]
    #[init(val = 500.)]
    turn_accel: f32,
    #[var]
    turn_speed: f32,
    #[export]
    #[init(val = 450.)]
    /// The maximum distance from the planet it is connected to
    max_grav_dist: f32,
    #[export]
    #[init(val = 1250.)]
    max_speed: f32,
    planet_data: Option<(Vector2, f32)>,
    #[var]
    pos_log: PackedVector2Array,
    #[var]
    time_log: PackedFloat64Array,
    #[var]
    orbitting: bool,
    /// Is true when it has fully orbitted the center
    #[var]
    orbitted: bool,
    base: Base<RigidBody2D>,
}

#[godot_api]
impl IRigidBody2D for GravObject {
    fn ready(&mut self,) {
        self.turn_speed = self.max_turn_speed
    }
}

#[godot_api]
impl GravObject {
    #[func]
    fn set_planet(&mut self, pos: Vector2, strength: f32) {
        self.planet_data = Some((pos, strength))
    }

    #[func]
    fn get_grav_center(&self) -> Vector2 {
        self.planet_data.unwrap().0
    }

    #[func]
    fn unset_planet(&mut self) {
        self.planet_data = None;
        self.orbitted = false;
        self.orbitting = false;
    }

    #[func]
    fn grav_physics_tick(&mut self, _delta: f64) {
        if let Some((center, grav_strength)) = self.planet_data {
            let global_pos = self.base().get_global_position();
            self.base_mut().apply_force(
                global_pos.direction_to(center) * grav_strength / global_pos.distance_to(center),
            );

            if !self.orbitted {
                if self.get_planet_circle(self.threshhold_range) {
                    self.orbitted = true
                }
            }
        } else {
            self.orbitted = false
        }
    }

    #[func]
    fn log_pos(&mut self, delta: f64) {
        let global_pos = self.base().get_global_position();
        self.pos_log.push(global_pos);
        self.time_log.push(delta);
    }

    #[func]
    fn get_log_with_res(&self, res: f32) -> PackedVector2Array {
        let mut log = self.pos_log.clone();
        log.reduce(Preciscion::Resolution(res));
        log
    }

    #[func]
    fn get_log_with_amount(&self, amount: u64) -> PackedVector2Array {
        let mut log = self.pos_log.clone();
        log.reduce(Preciscion::Amount(amount as usize));
        log
    }

    #[func]
    fn get_log_with_ignorant_amount(&self, amount: u64) -> PackedVector2Array {
        let mut log = self.pos_log.clone();
        log.reduce(Preciscion::IgnorantAmount(amount as usize));
        log
    }

    // Potentially change .unwrap to .unwrap_unchecked
    fn get_planet_circle(&self, threshhold: f32) -> bool {
        if let Some((center, _)) = self.planet_data {
            // maybe something better than averge time
            let n_back = (self.hold_time / self.time_log.average()).ceil() as usize;

            if !(self.pos_log.len() >= n_back) {
                // not long enough existence
                return false;
            };

            let init_val = self.pos_log.get(self.pos_log.len() - 1).unwrap().distance_to(center);

            let (mut min, mut max): (f32, f32) = (init_val, init_val);

            for i in self.pos_log.len()-n_back ..self.pos_log.len() - 2 {
                let dist = self.pos_log.get(i).unwrap().distance_to(center);
                if dist > max { max = dist } else if dist < min { min = dist }
            };

            godot_print!("range: {}", max - min);

            // TODO: make -* work
            // if the difference is bigger than threshhold fail
            max - min <= threshhold
        } else {
            // no center
            false
        }
    }
}

impl Logging<f64> for PackedFloat64Array {
    fn average(&self) -> f64 {
        let mut total = 0.;

        for f in self.to_vec() {
            total += f
        }

        total / self.len() as f64
    }
}

impl Logging<Vector2> for PackedArray<Vector2> {
    fn dedup(&mut self) {
        let mut vec = self.to_vec();

        vec.dedup();

        *self = PackedVector2Array::from(vec)
    }

    fn reduce(&mut self, precision: Preciscion) {
        match precision {
            Preciscion::IgnorantAmount(amount) => {
                let mut result = PackedVector2Array::new();

                self.dedup();

                let jump_size = self.len() / amount;

                for i in 0..amount {
                    result.push(self.get(i * jump_size).unwrap());
                }

                *self = result
            }
            Preciscion::Amount(amount) => {
                let mut result = PackedVector2Array::new();

                let jump_size = self.len() / amount;

                for i in 0..amount {
                    result.push(self.get(i * jump_size).unwrap());
                }

                *self = result
            }
            Preciscion::Resolution(res) => {
                let mut result = PackedVector2Array::new();

                let len = self.len();
                if len == 0 {
                    *self = result;
                    return;
                }

                let mut last_kept = self.get(0).unwrap();
                result.push(last_kept);

                for i in 1..len {
                    let point = self.get(i).unwrap();

                    if last_kept.distance_to(point) >= res {
                        result.push(point);
                        last_kept = point;
                    }
                }

                *self = result;
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Preciscion {
    Amount(usize),
    IgnorantAmount(usize),
    Resolution(f32),
}

trait Logging<T>
where
    T: PackedArrayElement,
{
    fn average(&self) -> T {
        unimplemented!()
    }
    /// Remove duplicates
    fn dedup(&mut self) {
        unimplemented!()
    }
    /// Reduce the array by a preciscion level
    fn reduce(&mut self, precision: Preciscion) {
        match precision {
            Preciscion::Amount(_) => unimplemented!(),
            Preciscion::IgnorantAmount(_) => unimplemented!(),
            Preciscion::Resolution(_) => unimplemented!(),
        }
    }
}
