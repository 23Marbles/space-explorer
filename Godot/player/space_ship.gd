extends GravObject

func set_center_vals(center: Vector2, strength: float):
	apply_impulse(
		(
			global_position.direction_to(center)
			* strength / global_position.distance_to(center)
			) / 10
		)
	set_planet(center, strength)

func unset_center_vals():
	unset_planet()

func _physics_process(_delta: float) -> void:
	apply_force((Vector2(0, Input.get_axis("forward", "down")) * max_speed).rotated(rotation))
	apply_torque(Input.get_axis("left", "right") * turn_speed)
	linear_velocity *= 0.99
	angular_velocity *= 0.9
	grav_physics_tick(_delta)
	log_pos()
