class_name Planet extends StaticBody2D

@export var strength_multiplier := 1.

signal planet_connected(center: Vector2, strength: float)
signal planet_disconnected()

@onready var texture_progress_bar: TextureProgressBar = $TextureProgressBar

var contains_mouse := false
var enacted := false

func _process(_delta: float) -> void:
	if enacted:
		texture_progress_bar.value = 100
	else:
		texture_progress_bar.value = 0
	
	if Input.is_action_just_pressed("click") && contains_mouse:
		planet_connected.emit(global_position, strength_multiplier * 100_000)
		enacted = true
	elif Input.is_action_just_released("click"):
		planet_disconnected.emit()
		enacted = false

func _on_mouse_entered() -> void:
	contains_mouse = true

func _on_mouse_exited() -> void:
	contains_mouse = false
