class_name Planet extends StaticBody2D

@export var strength_multiplier := 1.

signal planet_connected(center: Vector2, strength: float)
signal planet_disconnected()

var contains_mouse := false

func _process(_delta: float) -> void:
	if Input.is_action_just_pressed("click") && contains_mouse:
		planet_connected.emit(global_position, strength_multiplier * 100_000)
	elif Input.is_action_just_released("click"):
		planet_disconnected.emit()

func _on_mouse_entered() -> void:
	contains_mouse = true

func _on_mouse_exited() -> void:
	contains_mouse = false
