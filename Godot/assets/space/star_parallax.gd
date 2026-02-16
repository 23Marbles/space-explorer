@tool
class_name StarParallax2D extends Parallax2D

@export var reload := false:
	set(val):
		if val:
			reload_stars()

@export var max_scale: float = 1.0
@export var min_scale := 0.8
@export var star_quantity := 3

@export var star_textures: Array[Texture2D] = [load("res://assets/space/background_star.png")]

func reload_stars():
	for c in get_children():
		c.queue_free()
	
	var stars: Array[Star] = []
	
	for s in star_quantity:
		var star := Star.new()
		star.position = Vector2(randf_range(0, repeat_size.x), randf_range(0, repeat_size.y))
		star.scale = Vector2.ONE * randf_range(min_scale, max_scale)
		star.rotation = randf()
		
		star.texture = star_textures.pick_random()
		
		stars.push_back(star)
	
	var total_scale := Vector2.ZERO
	
	for star in stars:
		total_scale += star.scale
		add_child(star)
	
	scroll_scale = total_scale / star_quantity
	
	modulate.a = total_scale.x / star_quantity

func _ready() -> void:
	reload = true

func _init() -> void:
	z_index = -1
	repeat_size = Vector2.ONE * 2000
	repeat_times = 4
	reload_stars()
