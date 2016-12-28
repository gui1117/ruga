velocity = 30.0
time_to_reach_vmax = 0.1
weight = 1.0
zoom = 0.05

run_keys = {z="up", s="down", q="left", d="right"}
shoot_key = "mouseleft"

function is_orthogonal(a, b)
	if a == "up" or a == "down" then
		return b == "left" or b == "right"
	else
		return b == "up" or b == "down"
	end
end

last_run_angle = 0.0
run_dir_buf = {}

function input(state, scancode, code)
	if state == "pressed" then
		if run_keys[code] then run_key_pressed(run_keys[code])
		elseif code == shoot_key then set_player_shoot(true)
		elseif code == "r" then set_player_weapon("sniper", 0.4, 1, 1)
		end
	else
		if run_keys[code] then run_key_released(run_keys[code])
		elseif code == shoot_key then set_player_shoot(false)
		end
	end
	local a = run_dir_buf[1] or ""
end

function run_key_pressed(run_dir)
	local index = 1
	for i, v in ipairs(run_dir_buf) do
		if run_dir == v then
			return
		end
		index = index + 1
	end
	run_dir_buf[index] = run_dir
	update_player_run_dir()
end

function run_key_released(run_dir)
	local remove = false
	for i, value in ipairs(run_dir_buf) do
		if value == run_dir then
			remove = true
		end
		if remove then
			run_dir_buf[i] = run_dir_buf[i+1]
		end
	end
	if remove then
		update_player_run_dir()
	end
end

PI = 3.1415926535897932384626433832795
PI34 = 3/4*PI
PI4 = PI/4
PI2 = PI/2

function update_player_run_dir()
	local vertical = nil;
	local horizontal = nil;
	for _, value in ipairs(run_dir_buf) do
		if value == "right" or value == "left" then
			horizontal = value
		else
			vertical = value
		end
	end

	local angle = last_run_angle
	local strength = 1.0

	if vertical == "down" then
		if horizontal == "left" then
			angle = -PI34
		elseif horizontal == "right" then
			angle = -PI4
		else
			angle = -PI2
		end
	elseif vertical == "up" then
		if horizontal == "left" then
			angle = PI34
		elseif horizontal == "right" then
			angle = PI4
		else
			angle = PI2
		end
	else
		if horizontal == "left" then
			angle = PI
		elseif horizontal == "right" then
			angle = 0.0
		else
			strength = 0.0
		end
	end
	last_run_angle = angle

	set_player_force(angle, strength)
end

function mouse_moved(x, y)
	set_player_aim(math.atan2(y,x))
end

function mouse_wheel(horizontal, vertical)
	zoom = zoom + vertical*0.01
	update_zoom()
end

function update_zoom()
	set_zoom(zoom)
end

set_zoom(zoom)
add_wall(0.0, 0.0, 5.0, 10.0)
add_character(10.0, 10.0, 1.0, velocity, time_to_reach_vmax, weight)
set_player_weapon("sniper", 0.4, 1, 1)
fill_physic_world()
