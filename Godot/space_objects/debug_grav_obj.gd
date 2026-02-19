extends GravObject

var force: float

var connected_planet: Planet = Planet.new()

func _ready() -> void:
	force = 1

func _physics_process(delta: float) -> void:
	apply_force((Vector2(0, force) * speed).rotated(rotation))
	apply_torque((1 if orbit_clockwise else -1) * turn_speed)
	
	linear_velocity *= 0.99
	angular_velocity *= 0.9
	grav_physics_tick(delta)
	log_pos(delta)
