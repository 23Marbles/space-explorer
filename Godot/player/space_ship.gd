class_name Player extends GravObject

var force: float

var connected_planet: Planet = Planet.new()

var move_twd_max_turn_speed: bool

var last_transmitter: EnergySatelite

func connect_planet(planet: Planet):
	move_twd_max_turn_speed = true
	apply_impulse(
		(
			global_position.direction_to(planet.get_planet_center())
			* planet.get_grav_strength() / global_position.distance_to(planet.get_planet_center())
			) / 10
		)
	connected_planet = planet
	speed = entry_speed
	set_planet(planet.get_planet_center(), planet.get_grav_strength(), planet.in_atmosphere)

func _input(event: InputEvent) -> void:
	if event.is_action_pressed("right_click") && orbitted:
		#spawn_inherited_grav_object(load("res://space_objects/solar_panel.tscn"))
		var scene: EnergySatelite
		
		if last_transmitter:
			if !connected_planet.input_node:
				scene = preload("res://space_objects/energy_satelite.tscn").instantiate()
				scene.input_from = last_transmitter
				
				last_transmitter = scene
			else:
				scene = preload("res://space_objects/energy_satelite.tscn").instantiate()
				scene.input_from = last_transmitter
				scene.output_to = connected_planet.input_node
				
				last_transmitter = null
			
		else:
			scene = preload("res://space_objects/solar_panel.tscn").instantiate()
			last_transmitter = scene
		
		scene.connected_planet = connected_planet
		scene.global_position = global_position
		
		
		get_parent().add_child(scene)
	
	if event.is_action_pressed("debug_press"):
		if has_planet():
			
			break_orbital_connection()
			
			if !has_planet():
				orbit_clockwise = !orbit_clockwise
				move_twd_max_turn_speed = true

@onready var camera_2d: Camera2D = $Camera2D

func _physics_process(delta: float) -> void:
	set_atmosphere(connected_planet.in_atmosphere)
	set_clicking(connected_planet.clicked)
	
	if move_twd_max_turn_speed:
		turn_speed = move_toward(turn_speed, 40000., delta * turn_accel)
		if turn_speed == max_turn_speed:
			move_twd_max_turn_speed = false
	
	if has_planet():
		hold_time = connected_planet.hold_time
		threshhold_range = connected_planet.threshhold_range
		max_turn_speed = connected_planet.max_turn_speed
	
	if !orbitting:
		var dir = Input.get_axis("forward", "down")
		
		if dir > 0:
			force = 1
		elif dir < 0:
			force = -1
	
	var rot = Input.get_axis("left", "right")
	
	if !orbitted:
		camera_2d.ignore_rotation = true
		
		if rot > 0:
			orbit_clockwise = true
		elif rot < 0:
			orbit_clockwise = false
	else:
		camera_2d.ignore_rotation = false
		orbitting = true
		
		turn_speed -= rot * turn_accel * delta * (-1 if orbit_clockwise else 1)
	
	turn_speed = min(turn_speed, max_turn_speed)
	
	apply_force((Vector2(0, force) * speed).rotated(rotation))
	apply_torque((1 if orbit_clockwise else -1) * turn_speed)
	
	linear_velocity *= 0.99
	angular_velocity *= 0.9
	grav_physics_tick(delta)
	log_pos(delta)
