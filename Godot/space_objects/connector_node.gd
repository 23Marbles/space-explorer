@tool
class_name ConnectionNode extends Area2D

enum ConnectionType{Output, Input}

@export var reload: bool = false:
	set(v):
		if v:
			_reload()

@export var connection_type: ConnectionType

@export var outline_color: Color = Color.WHITE

@export var outline_size: float = 10

@export var radius: float = 32

var connections: Array[ConnectionNode]

var useful_connections: Array[ConnectionNode]

@export var useful: bool

var connected: bool

@onready var parent: EnergySatelite = $"../.."

var child_sprite: Sprite2D

func _ready() -> void:
	_reload()

func _init() -> void:
	reload_sprite()
	reload_area()

func _draw() -> void:
	draw_connections()

func draw_connections():
	for c in connections:
		draw_line(Vector2.ZERO, to_local(c.global_position), Color.YELLOW)

func reload_sprite():
	for c in get_children(true):
		if c is Sprite2D:
			c.queue_free()
	
	var sprite := Sprite2D.new()
	
	var texture := GradientTexture2D.new()
	texture.fill = GradientTexture2D.FILL_RADIAL
	texture.fill_from = Vector2(0.5, 0.5)
	texture.fill_to = Vector2(0.5, 0.0)
	
	texture.width = radius * 2
	texture.height = radius * 2
	
	var grad := Gradient.new()
	
	var outline_offset := outline_size / radius
	
	grad.offsets = [
		0.0,
		0.97 - outline_offset - 0.03,
		0.97 - outline_offset,
		0.97,
		1
	]
	
	var alpha_outline = outline_color
	alpha_outline.a8 = 0
	
	grad.colors = [
		get_color(),
		get_color(),
		outline_color,
		outline_color,
		alpha_outline
	]
	
	texture.gradient = grad
	sprite.texture = texture
	
	add_child(sprite)
	
	child_sprite = sprite

func reload_area():
	for c in get_children(true):
		if c is CollisionShape2D:
			c.queue_free()
	
	var collision_shape := CollisionShape2D.new()
	
	collision_shape.shape = CircleShape2D.new()
	collision_shape.shape.radius = radius
	
	add_child(collision_shape)

func _reload():
	reload_sprite()
	reload_area()
	
	match connection_type:
		ConnectionType.Output:
			collision_layer = 32
		ConnectionType.Input:
			collision_layer = 16

func get_color() -> Color:
	match connection_type:
		ConnectionType.Output:
			return Color.LIGHT_BLUE
		ConnectionType.Input:
			if useful:
				if !connected:
					return Color.FIREBRICK
				else:
					return Color.AQUAMARINE
			else:
				if !connected:
					return Color.INDIAN_RED
				else:
					return Color.DARK_SEA_GREEN
	
	return Color.WHITE

func _process(_delta: float) -> void:
	update_useful_connections()
	child_sprite.texture.gradient.colors[0] = get_color()
	child_sprite.texture.gradient.colors[1] = get_color()

func update_useful_connections():
	useful_connections.clear()
	
	for c in connections:
		if c.useful:
			useful_connections.push_back(c)
	
	if connection_type == ConnectionType.Output:
		useful = not useful_connections.is_empty()

### Returns loss
func transfer(amount: int) -> int:
	match connection_type:
		ConnectionType.Output:
			return give(amount)
			
		ConnectionType.Input:
			return recieve(amount)
			
	
	return 0

func connect_to(other: ConnectionNode):
	match other.connection_type:
		ConnectionType.Output:
			if connection_type == ConnectionType.Input:
				connections.push_back(other)
				other.connections.push_back(self)
		ConnectionType.Input:
			if connection_type == ConnectionType.Output:
				connections.push_back(other)
				other.connections.push_back(self)
			

func give(amount: int) -> int:
	update_useful_connections()
	
	var individual_amount = floor(float(amount) / useful_connections.size())
	
	var loss: int = 0
	
	for c in useful_connections:
		loss += c.recieve(individual_amount)
	
	return loss

func recieve(amount: int) -> int:
	return parent.recieve_energy(amount)
