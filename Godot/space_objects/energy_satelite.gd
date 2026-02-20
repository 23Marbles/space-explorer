class_name EnergySatelite extends Satelite

@export var output_to: EnergySatelite:
	set(v):
		output_to = v
		if output_node && output_to.input_node:
			output_node.connect_to(output_to.input_node)

@export var input_from: EnergySatelite:
	set(v):
		input_from = v
		if output_node && input_from.output_node:
			input_node.connect_to(input_from.output_node)

@export var output_node: ConnectionNode
@export var input_node: ConnectionNode

var energy: int

var useful: bool

func _process(_delta: float) -> void:
	if useful && input_node:
		input_node.useful = true
	elif output_node && output_node.useful:
		useful = true
	
	if output_node:
		energy -= output_node.transfer(energy)

func _ready() -> void:
	super._ready()
	
	if output_to && output_node:
		if output_to.input_node:
			output_node.connect_to(output_to.input_node)

### returns amount of energy taken
func recieve_energy(amount: int) -> int:
	energy += amount
	return amount
