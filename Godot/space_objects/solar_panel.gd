extends EnergySatelite

@onready var solar_panel: Sprite2D = $SolarPanel
@onready var collision_shape_2d: CollisionShape2D = $CollisionShape2D

## time between generting 1 peice of energy
@export var cooldown_time: float = 1

func _ready() -> void:
	super._ready()
	var timer = Timer.new()
	timer.wait_time = cooldown_time
	timer.autostart = true
	timer.one_shot = false
	timer.timeout.connect(generate_energy)
	add_child(timer)

func generate_energy():
	energy += 1

func _process(_delta: float) -> void:
	super._process(_delta)
	output_node.give(energy)
	
	solar_panel.look_at(center)
	collision_shape_2d.look_at(center)
	
	energy -= output_node.give(energy)

func die():
	queue_free()

func _on_body_hit(_body: Node) -> void:
	die()
