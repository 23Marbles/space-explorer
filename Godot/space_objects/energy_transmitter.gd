extends EnergySatelite

func _process(_delta: float) -> void:
	super._process(_delta)
	
	if output_node.connections.size() > 0:
		output_node.queue_redraw()
		$TransmitterHead.look_at(output_node.connections[0].global_position)
		$TransmitterHead.rotation_degrees -= 90
	if input_node.connections.size() > 0:
		input_node.queue_redraw()
		$TransmitterHead2.look_at(input_node.connections[0].global_position)
		$TransmitterHead2.rotation_degrees += 90
	
	$Transmitter.rotation = $TransmitterHead.rotation + $TransmitterHead2.rotation
