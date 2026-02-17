@tool
class_name StarParallax2D extends Parallax2D

@export var reload := false:
	set(val):
		if val:
			reload_stars()

@export var max_scale: float = 1.0
@export var min_scale := 0.8
@export var star_quantity := 3

@export var overload_star_data: bool

@export var star_textures: SpaceBackground = load("res://assets/space/space_background.tres")

func reload_stars():
	for c in get_children():
		c.queue_free()
	
	var stars: Array[Star]
	
	if overload_star_data:
		stars = star_textures.generate_stars_quantity(repeat_size, star_quantity)
		
		for s in stars:
			s.scale = Vector2.ONE * randf_range(min_scale, max_scale)
	else:
		stars = star_textures.generate_stars(repeat_size)
	
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
