use std::fmt::Debug;

use godot::{
    classes::{IRigidBody2D, RigidBody2D},
    meta::PackedArrayElement,
    obj::Base,
    prelude::*,
};

enum ConnectionChange {
    Atmosphere(bool),
    Clicking(bool),
    Orbital(bool),
}

#[derive(Debug, Default, Clone, Copy)]
struct ConnectionStatus {
    in_atmosphere: bool,
    planet_clicked: bool,
    orbital_connection: bool,
}

impl ConnectionStatus {
    /// # Returns
    /// Returns whether you should leave the planet
    fn connection_change(&mut self, change: ConnectionChange) -> bool {
        match change {
            ConnectionChange::Atmosphere(value) => {
                self.in_atmosphere = value;
                self.should_leave()
            }
            ConnectionChange::Clicking(value) => {
                self.planet_clicked = value;
                self.should_leave()
            }
            ConnectionChange::Orbital(value) => {
                self.orbital_connection = value;
                self.should_leave()
            }
        }
    }

    fn should_leave(&self) -> bool {
        // if clicked definetely shouldn't leave
        if self.planet_clicked {
            false
        } else {
            // if not in atmoshphere leave unless planet clicked
            if self.orbital_connection {
                !self.in_atmosphere
            } else {
                // if not clicked
                true
            }
        }
    }
}

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
    #[init(val = 1500.)]
    exit_speed: f32,
    #[export]
    #[init(val = 1000.)]
    entry_speed: f32,
    #[var]
    speed: f32,
    planet_data: Option<(Vector2, f32)>,
    planet_connection: Option<ConnectionStatus>,
    #[var]
    pos_log: PackedVector2Array,
    #[var]
    time_log: PackedFloat64Array,
    #[var]
    orbitting: bool,
    #[var]
    orbit_clockwise: bool,
    /// Is true when it has fully orbitted the center
    #[var]
    orbitted: bool,
    base: Base<RigidBody2D>,
}

impl Debug for GravObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GravObject")
            .field("threshhold_range", &self.threshhold_range)
            .field("hold_time", &self.hold_time)
            .field("max_turn_speed", &self.max_turn_speed)
            .field("turn_accel", &self.turn_accel)
            .field("turn_speed", &self.turn_speed)
            .field("exit_speed", &self.exit_speed)
            .field("entry_speed", &self.entry_speed)
            .field("speed", &self.speed)
            .field("planet_data", &self.planet_data)
            .field("planet_connection", &self.planet_connection)
            //.field("pos_log", &self.pos_log)
            //.field("time_log", &self.time_log)
            .field("orbitting", &self.orbitting)
            .field("orbitted", &self.orbitted)
            .field("base", &self.base)
            .finish()
    }
}

#[godot_api]
impl IRigidBody2D for GravObject {
    fn ready(&mut self) {
        self.turn_speed = self.max_turn_speed;
        self.speed = self.exit_speed;
    }
}

#[godot_api]
impl GravObject {
    fn inherit(&mut self, other: &GravObject) {
        self.speed = other.speed;
        self.turn_speed = other.turn_speed;
        self.planet_connection = other.planet_connection;
        self.planet_data = other.planet_data;
        self.orbitting = other.orbitting;
        self.orbitted = other.orbitted;

        // invert this cus...?
        self.orbit_clockwise = !other.orbit_clockwise;

        self.base_mut()
            .set_linear_velocity(other.base().get_linear_velocity());
        self.base_mut()
            .set_angular_velocity(other.base().get_angular_velocity());

        self.base_mut()
            .set_global_position(other.base().get_global_position());
        self.base_mut()
            .set_global_rotation(other.base().get_global_rotation());
    }

    #[func]
    fn spawn_inherited_grav_object(&mut self, scene: Gd<PackedScene>) {
        if let Some(mut grav_obj) = scene.try_instantiate_as::<GravObject>() {
            let mut parent = self
                .base()
                .get_parent()
                .expect("no parent, cannot spawn grav_obj above target");

            let mut rust_grav_obj = grav_obj.bind_mut();

            rust_grav_obj.inherit(self);

            parent.add_child(&rust_grav_obj.to_gd());
        }
    }

    #[func]
    fn has_planet(&self) -> bool {
        self.planet_data.is_some()
    }

    #[func]
    fn set_planet(&mut self, pos: Vector2, strength: f32, in_atmosphere: bool) {
        self.speed = self.entry_speed;
        self.planet_data = Some((pos, strength));
        self.planet_connection = Some(ConnectionStatus {
            in_atmosphere,
            planet_clicked: true,
            orbital_connection: false,
        })
    }

    #[func]
    fn get_grav_center(&self) -> Vector2 {
        self.planet_data.unwrap().0
    }

    #[func]
    fn set_atmosphere(&mut self, atmosphere: bool) {
        if self.planet_connection.is_none() {
            return;
        }
        if self
            .planet_connection
            .as_mut()
            .unwrap()
            .connection_change(ConnectionChange::Atmosphere(atmosphere))
        {
            self.unset_planet();
        }
    }

    #[func]
    fn leave_atmosphere(&mut self) {
        if self.planet_connection.is_none() {
            return;
        }
        if self
            .planet_connection
            .as_mut()
            .unwrap()
            .connection_change(ConnectionChange::Atmosphere(false))
        {
            self.unset_planet();
        }
    }

    #[func]
    fn enter_atmosphere(&mut self) {
        self.planet_connection.as_mut().unwrap().in_atmosphere = true
    }

    #[func]
    fn break_orbital_connection(&mut self) {
        if self.planet_connection.is_none() {
            return;
        }
        if self
            .planet_connection
            .as_mut()
            .unwrap()
            .connection_change(ConnectionChange::Orbital(false))
        {
            self.unset_planet();
        }
    }

    #[func]
    fn set_clicking(&mut self, clicking: bool) {
        if self.planet_connection.is_none() {
            return;
        }
        if self
            .planet_connection
            .as_mut()
            .unwrap()
            .connection_change(ConnectionChange::Clicking(clicking))
        {
            self.unset_planet();
        }
    }

    #[func]
    fn click_planet(&mut self) {
        self.planet_connection.as_mut().unwrap().planet_clicked = true
    }

    #[func]
    fn stop_clicking_planet(&mut self) {
        if self.planet_connection.is_none() {
            return;
        }
        if self
            .planet_connection
            .as_mut()
            .unwrap()
            .connection_change(ConnectionChange::Clicking(false))
        {
            self.unset_planet();
        }
    }

    #[func]
    fn unset_planet(&mut self) {
        self.planet_data = None;
        self.orbitted = false;
        self.orbitting = false;
        self.speed = self.exit_speed;
        self.planet_connection = None
    }

    #[func]
    fn grav_physics_tick(&mut self, _delta: f64) {
        if let Some((center, grav_strength)) = self.planet_data {
            let global_pos = self.base().get_global_position();
            self.base_mut().apply_force(
                global_pos.direction_to(center) * grav_strength / global_pos.distance_to(center),
            );

            if !self.orbitted {
                if self.get_planet_circle() {
                    self.planet_connection.as_mut().unwrap().orbital_connection = true;
                    self.orbitted = true
                }
            }
        } else {
            self.orbitted = false;
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
    fn get_planet_circle(&self) -> bool {
        if let Some((center, _)) = self.planet_data {
            if self.pos_log.len() < 2 {
                return false;
            }

            let n_back = (self.hold_time / self.time_log.average()).ceil() as usize;

            if self.pos_log.len() <= n_back {
                return false;
            }

            let vec_pos_log = self.pos_log.to_vec();

            // Reverse iteration without cloning (more efficient)
            let recent = &vec_pos_log[vec_pos_log.len() - n_back..];

            let mut min_dist;
            let mut max_dist;

            // Initial distance
            let first_dist = recent[0].distance_to(center);
            min_dist = first_dist;
            max_dist = first_dist;

            let mut total_angle_change = 0.0;
            let mut prev_angle = angle_from_center(recent[0], center);

            for pos in &recent[1..] {
                let dist = pos.distance_to(center);

                if dist > max_dist {
                    max_dist = dist;
                } else if dist < min_dist {
                    min_dist = dist;
                }

                // Radial stability check
                if max_dist - min_dist > self.threshhold_range {
                    return false;
                }

                // Angular movement tracking
                let angle = angle_from_center(*pos, center);
                let delta = smallest_angle_diff(prev_angle, angle);
                total_angle_change += delta.abs();
                prev_angle = angle;
            }

            const MIN_TOTAL_ANGLE: f32 = 2.;

            // Reject sitting still or tiny rocking
            if total_angle_change < MIN_TOTAL_ANGLE {
                return false;
            }

            true
        } else {
            false
        }
    }
}

fn angle_from_center(pos: Vector2, center: Vector2) -> f32 {
    (pos.y - center.y).atan2(pos.x - center.x)
}

fn smallest_angle_diff(a: f32, b: f32) -> f32 {
    use std::f32::consts::PI;
    let mut diff = b - a;
    while diff > PI {
        diff -= 2.0 * PI;
    }
    while diff < -PI {
        diff += 2.0 * PI;
    }
    diff
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
