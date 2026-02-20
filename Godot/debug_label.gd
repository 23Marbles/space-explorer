extends Label

@onready var space_ship: GravObject = $"../.."

func _process(_delta: float) -> void:
	var dist: float = space_ship.global_position.distance_to(space_ship.get_grav_center())
	text = str(round(dist))
