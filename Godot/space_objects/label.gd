extends Label

@onready var energy_satelite: EnergySatelite = $".."

func _process(_delta: float) -> void:
	text = str(energy_satelite.energy)
