@tool
class_name SpaceBackground extends Resource

@export var max_scale: float = 1.0
@export var min_scale := 0.8
@export var star_quantity := 3

@export var star_textures: Array[Texture2D] = [load("res://assets/space/background_star.png")]

func generate_stars_quantity(repeat_size: Vector2, quantity: int) -> Array[Star]:
	var stars: Array[Star] = []
	
	for s in quantity:
		var star := Star.new()
		star.position = Vector2(randf_range(0, repeat_size.x), randf_range(0, repeat_size.y))
		star.scale = Vector2.ONE * randf_range(min_scale, max_scale)
		star.rotation = randf()
		
		star.texture = star_textures.pick_random()
		
		stars.push_back(star)
	
	return stars

func generate_stars(repeat_size: Vector2) -> Array[Star]:
	var stars: Array[Star] = []
	
	for s in star_quantity:
		var star := Star.new()
		star.position = Vector2(randf_range(0, repeat_size.x), randf_range(0, repeat_size.y))
		star.scale = Vector2.ONE * randf_range(min_scale, max_scale)
		star.rotation = randf()
		
		star.texture = star_textures.pick_random()
		
		stars.push_back(star)
	
	return stars
