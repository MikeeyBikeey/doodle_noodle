extends RigidBody2D

onready var sprite := $Sprite
onready var collision := $CollisionShape2D

func update_object(object: Dictionary):
	var texture := ImageTexture.new()
	texture.create_from_image(object.image)
	sprite.texture = texture
	
	var width: float = object.right - object.left
	var height: float = object.bottom - object.top
	collision.shape.extents = Vector2(width, height) / 2.0
