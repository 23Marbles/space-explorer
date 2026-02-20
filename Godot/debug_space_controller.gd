extends Node2D

@onready var space_ship := $SpaceShip
@onready var planets: Node2D = $Planets

func _ready() -> void:
	for c in planets.get_children():
		if c is Planet:
			c.connect("planet_connected", connect_planet)

func _process(_delta: float) -> void:
	queue_redraw()

#func _unhandled_input(event: InputEvent) -> void:
	#if event.is_action_pressed("ui_accept"):
	#	$SpaceShip/Camera2D.zoom = Vector2.ONE * 0.1

func _draw() -> void:
	draw_polyline(space_ship.get_log_with_res(7), Color.WHITE, 10)

func connect_planet(planet: Planet):
	space_ship.connect_planet(planet)
