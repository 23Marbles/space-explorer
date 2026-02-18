extends Label

@onready var space_ship: GravObject = $"../.."

func _process(_delta: float) -> void:
	var dist: float = space_ship.turn_speed
	text = str(round(dist))
