use godot::{
    classes::RigidBody2D,
    meta::PackedArrayElement,
    obj::Base,
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=RigidBody2D, init)]
struct GravObject {
    #[export]
    #[init(val = 100.)]
    turn_speed: f32,
    #[export]
    #[init(val = 450.)]
    /// The maximum distance from the planet it is connected to
    max_grav_dist: f32,
    #[export]
    #[init(val = 300.)]
    max_speed: f32,
    planet_data: Option<(Vector2, f32)>,
    #[var]
    log: PackedVector2Array,
    base: Base<RigidBody2D>,
}

#[godot_api]
impl GravObject {
    #[func]
    fn set_planet(&mut self, pos: Vector2, strength: f32) {
        self.planet_data = Some((pos, strength))
    }

    #[func]
    fn unset_planet(&mut self) {
        self.planet_data = None
    }

    #[func]
    fn grav_physics_tick(&mut self, _delta: f64) {
        if let Some((center, grav_strength)) = self.planet_data {
            let global_pos = self.base().get_global_position();
            self.base_mut().apply_force(
                global_pos.direction_to(center) * grav_strength / global_pos.distance_to(center),
            );
        }
    }

    #[func]
    fn log_pos(&mut self) {
        let global_pos = self.base().get_global_position();
        self.log.push_no_con_dup(global_pos);
    }

    #[func]
    fn log_pos_ignorant(&mut self) {
        let global_pos = self.base().get_global_position();
        self.log.push(global_pos);
    }

    #[func]
    fn log_pos_strict(&mut self) {
        let global_pos = self.base().get_global_position();
        self.log.push_no_dup(global_pos);
    }

    #[func]
    fn get_log_with_res(&self, res: f32) -> PackedVector2Array {
        let mut log = self.log.clone();
        log.reduce(Preciscion::Resolution(res));
        log
    }

    #[func]
    fn get_log_with_amount(&self, amount: u64) -> PackedVector2Array {
        let mut log = self.log.clone();
        log.reduce(Preciscion::Amount(amount as usize));
        log
    }
}

impl Logging<Vector2> for PackedArray<Vector2> {
    fn push_no_dup(&mut self, value: Vector2) {
        if !self.contains(value.clone()) {
            self.push(value);
        }
    }

    fn push_no_con_dup(&mut self, value: Vector2) {
        if self.len() == 0 {
            self.push(value);
        } else if self.get(self.len() - 1).unwrap() != value.clone() {
            self.push(value);
        }
    }

    fn reduce(&mut self, precision: Preciscion) {
        match precision {
            Preciscion::Amount(amount) => {
                let mut result = PackedVector2Array::new();

                let jump_size = self.len() / amount;

                for i in 0..amount {
                    result.push(self.get(i * jump_size).unwrap());
                }
            },
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
    Resolution(f32),
}

trait Logging<T>
where
    T: PackedArrayElement,
{
    /// Push a number with out any duplication
    fn push_no_dup(&mut self, value: T);
    /// Push a number with out consecutive duplication
    fn push_no_con_dup(&mut self, value: T);
    /// Reduce the array by a preciscion level
    fn reduce(&mut self, precision: Preciscion);
}
