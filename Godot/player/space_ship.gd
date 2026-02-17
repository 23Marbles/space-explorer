extends GravObject

var orbit_clockwise: bool
var force: float

func set_center_vals(center: Vector2, strength: float):
	apply_impulse(
		(
			global_position.direction_to(center)
			* strength / global_position.distance_to(center)
			) / 10
		)
	set_planet(center, strength)

var try_unset: bool = false

func unset_center_vals():
	if !orbitted:
		unset_planet()
	else:
		try_unset = true

func _unhandled_input(event: InputEvent) -> void:
	if event.is_action_pressed("debug_press"):
		if try_unset:
			orbit_clockwise = !orbit_clockwise
			unset_planet()
			try_unset = false

@onready var camera_2d: Camera2D = $Camera2D

func _physics_process(delta: float) -> void:
	if orbitted:
		camera_2d.ignore_rotation = false
		orbitting = true
	else:
		camera_2d.ignore_rotation = true
	
	if !orbitting:
		var dir = Input.get_axis("forward", "down")
		
		if dir > 0:
			force = 1
		elif dir < 0:
			force = -1
	
	var rot = Input.get_axis("left", "right")
	
	if !orbitted:
		if rot > 0:
			orbit_clockwise = true
		elif rot < 0:
			orbit_clockwise = false
	else:
		turn_speed -= rot * turn_accel * delta
	
	turn_speed = min(turn_speed, max_turn_speed)
	
	apply_force((Vector2(0, force) * max_speed).rotated(rotation))
	apply_torque((1 if orbit_clockwise else -1) * turn_speed)
	
	linear_velocity *= 0.99
	angular_velocity *= 0.9
	grav_physics_tick(delta)
	log_pos(delta)
