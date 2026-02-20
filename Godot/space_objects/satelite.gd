class_name Satelite extends RigidBody2D

@export var speed: float = 1000

var connected_planet: Planet = Planet.new()

var center: Vector2

func _ready() -> void:
	center = connected_planet.get_planet_center()
	# Distance from center
	radius = global_position.distance_to(center)
	
	# Angle from center
	angle = (global_position - center).angle()

var angle: float
var radius: float

func _physics_process(delta: float) -> void:
	angle += (speed * delta) / radius
	global_position = center + Vector2.RIGHT.rotated(angle) * radius
	
