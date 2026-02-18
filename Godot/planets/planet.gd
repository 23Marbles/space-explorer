class_name Planet extends StaticBody2D

@export var strength_multiplier := 1.

signal planet_connected(planet: Planet)

@onready var texture_progress_bar: TextureProgressBar = $TextureProgressBar

var contains_mouse := false

var in_atmosphere := false
var clicked := false

func get_grav_strength() -> float:
	return strength_multiplier * 100_000

func get_planet_center() -> Vector2:
	return global_position

func _process(_delta: float) -> void:
	if clicked:
		texture_progress_bar.value = 100
	else:
		texture_progress_bar.value = 0
	
	if Input.is_action_just_pressed("click") && contains_mouse:
		clicked = true
		planet_connected.emit(self)
	elif Input.is_action_just_released("click"):
		clicked = false
	

func _on_mouse_entered() -> void:
	contains_mouse = true

func _on_mouse_exited() -> void:
	contains_mouse = false

func _on_area_2d_body_exited(body: Node2D) -> void:
	if body is Player:
		in_atmosphere = false

func _on_area_2d_body_entered(body: Node2D) -> void:
	if body is Player:
		in_atmosphere = true
