extends Node2D

@onready var space_ship := $SpaceShip
@onready var planets: Node2D = $Planets

func _ready() -> void:
	for c in planets.get_children():
		if c is Planet:
			c.connect("planet_connected", connect_planet)
			c.connect("planet_disconnected", disconnect_planet)

func _process(_delta: float) -> void:
	queue_redraw()

func _draw() -> void:
	draw_polyline(space_ship.get_log_with_res(700.), Color.WHITE)

func connect_planet(center: Vector2, strength: float):
	space_ship.set_center_vals(center, strength)

func disconnect_planet():
	space_ship.unset_center_vals()
