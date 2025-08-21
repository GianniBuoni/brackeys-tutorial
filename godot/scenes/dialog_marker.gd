extends Area2D

@export var dialog_resource: DialogueResource
@export var dialog_start: String = "start"


func _on_body_entered(_body: Node2D) -> void:
	_activate()
	return

func _activate() -> void:
	DialogueManager.show_dialogue_balloon(dialog_resource, dialog_start)
	return
