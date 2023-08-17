extends Node

const ImageSearch := preload("res://image_search.gdns")
const Doodle := preload("res://doodle/doodle.tscn")

var camera_capture = null
onready var world := $World

onready var left_collision = $"%LeftCollision"
onready var right_collision = $"%RightCollision"
onready var top_collision = $"%TopCollision"
onready var bottom_collision = $"%BottomCollision"

func _ready():
	VisualServer.set_default_clear_color(Color("C8BDA9"))
	
	# Sets up default phone camera settings
	if Engine.has_singleton("GodotGetImage"):
		camera_capture = Engine.get_singleton("GodotGetImage")
		camera_capture.setOptions({
			image_width = 3024,
			image_height = 4032,
			image_format = "jpg",
			use_front_camera = true,
			auto_rotate_image = false,
		})
		camera_capture.connect("image_request_completed", self, "on_image_request_completed")
	
	# Sets up wall collisions
	var PADDING: float = 50.0
	var size: Vector2 = $Sprite.get_viewport().size
	left_collision.position = Vector2(-PADDING, size.y / 2.0)
	left_collision.shape.extents = Vector2(PADDING, size.y / 2.0 + PADDING)
	right_collision.position = Vector2(size.x + PADDING, size.y / 2.0)
	right_collision.shape.extents = Vector2(PADDING, size.y / 2.0 + PADDING)
	top_collision.position = Vector2(size.x / 2.0, -PADDING)
	top_collision.shape.extents = Vector2(size.x / 2.0 + PADDING, PADDING)
	bottom_collision.position = Vector2(size.x / 2.0, size.y + PADDING)
	bottom_collision.shape.extents = Vector2(size.x / 2.0 + PADDING, PADDING)


func _process(_delta):
	update_gravity_based_on_accelerometer()

# Updates the world gravity based on the phone's accelerometer.
func update_gravity_based_on_accelerometer():
	var accel: Vector3 = Input.get_accelerometer()
	if accel != Vector3.ZERO:
		var GRAVITY: float = 9.807 # The force of Earth's gravity is 9.807 m/s²
		var SPEED_MULTIPLIER: float = 12.0
		var gravity_vector = (Vector2(accel.x, -accel.y) / (GRAVITY / 2.0)) * SPEED_MULTIPLIER
		Physics2DServer.area_set_param(get_viewport().world_2d.get_space(), Physics2DServer.AREA_PARAM_GRAVITY_VECTOR, gravity_vector)

func create_objects():
	var image: Image = $Sprite.texture.get_data()
	image.convert(Image.FORMAT_RGBA8)
	
	# Searches the image
	var start = Time.get_ticks_msec()
	var image_search = ImageSearch.new()
	var objects = image_search.find_objects(image)
	print("Image search time: ", (Time.get_ticks_msec() - start) / 1000.0)
	
	# Instances the objects
	for obj in objects:
		# Image objects must be big enough
		if obj.image.get_width() < 5 || obj.image.get_height() < 5:
			continue
		
		var doodle := Doodle.instance()
		doodle.position.x = obj.left + obj.image.get_width() / 2.0
		doodle.position.y = obj.top + obj.image.get_height() / 2.0
		world.add_child(doodle)
		doodle.update_object(obj)
	
	# Updates the new texture
	var texture := ImageTexture.new()
	texture.create_from_image(image)
	$Sprite.texture = texture

func clear_world():
	for child in world.get_children():
		child.queue_free()

func _input(event: InputEvent):
	if event.is_action_pressed("toggle_fullscreen"):
		OS.window_fullscreen = !OS.window_fullscreen
	
	if event is InputEventScreenTouch:
		if event.pressed && Time.get_ticks_msec() > 2 * 1000:
			var size = $Sprite.get_viewport().size
			if event.position.y < size.y / 4.0 * 3.0:
				camera_capture.resendPermission()
				camera_capture.getCameraImage()
	
	if event is InputEventKey:
		if event.is_action_pressed("ui_accept"):
			create_objects()

func on_image_request_completed(image_buffers: Dictionary):
	for buffer in image_buffers.values():
		var image = Image.new()
		var error = image.load_jpg_from_buffer(buffer)
		
		if error != OK:
			push_error("Error loading camera's image buffer: " + String(error))
		else:
			var size = $Sprite.get_viewport().size
			var aspect = max(size.x / image.get_width(), size.y / image.get_height())
			
			image.resize(image.get_width() * aspect, image.get_height() * aspect, Image.INTERPOLATE_BILINEAR)
			image.blit_rect(image, Rect2((image.get_width() - size.x) / 2.0, (image.get_height() - size.y) / 2.0, size.x, size.y), Vector2.ZERO)
			image.crop(size.x, size.y)
			
			yield(get_tree(), "idle_frame")
			var texture = ImageTexture.new()
			texture.create_from_image(image, 0)
			$Sprite.texture = texture
			
			clear_world()
			create_objects()
