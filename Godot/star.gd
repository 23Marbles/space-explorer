class_name Star
extends Sprite2D

@export var length_multiplier := 2

var rand_phase := 0.0
var base_energy := 3.0

var twinkle_time := 10.

func _ready() -> void:
	rand_phase = randf() * TAU

var twinkling := false

func _process(_delta: float) -> void:
	var t = Time.get_ticks_msec() * 0.001
	var twinkle = length_multiplier + length_multiplier * sin(t + rand_phase)
	twinkle = min(twinkle, 1)
	modulate = Color(base_energy,
					 base_energy,
					 base_energy,
					 twinkle)
