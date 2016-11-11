run_directions = {z="up", s="down", q="left", d="right"}
velocity = 10.0

function ipairs(table)
	local i = 0
	return function()
		i = i + 1
		if table[i] ~= nil then
			return i, table[i]
		end
	end
end

function is_orthogonal(a, b)
	if a == "up" or a == "down" then
		return b == "left" or b == "right"
	else
		return b == "up" or b == "down"
	end
end

run_direction_buf = {}

function table_contains(vec, element)
	for _, value in ipairs(vec) do
		if value == element then
			return true
		end
	end
	return false
end

x0 = 0.6
y0 = 0.6
x1 = 0.6
y1 = 0.6

function input(state, scancode, code)
	if state == "pressed" then
		if code == "z" then
			y0 = y0 + 0.7
		end
		if code == "s" then
			y0 = y0 - 0.7
		end
		if code == "d" then
			x0 = x0 + 0.7
		end
		if code == "q" then
			x0 = x0 - 0.7
		end
		if code == "t" then
			y1 = y1 + 0.7
		end
		if code == "g" then
			y1 = y1 - 0.7
		end
		if code == "h" then
			x1 = x1 + 0.7
		end
		if code == "f" then
			x1 = x1 - 0.7
		end
		debug_raycast(x0,y0,x1,y1)
		if run_directions[code] then run_direction_pressed(run_directions[code]) end
	else
		if run_directions[code] then run_direction_released(run_directions[code]) end
	end
end

function run_direction_pressed(run_direction)
	local index = 1
	for i, v in ipairs(run_direction_buf) do
		if run_direction == v then
			return
		end
		index = index + 1
	end
	run_direction_buf[index] = run_direction
	update_player_direction()
end

function run_direction_released(run_direction)
	local remove = false
	for i, value in ipairs(run_direction_buf) do
		if value == run_direction then
			remove = true
		end
		if remove then
			run_direction_buf[i] = run_direction_buf[i+1]
		end
	end
	if remove then
		update_player_direction()
	end
end

function update_player_direction()
	local x = 0
	local y = 0
	for _, value in ipairs(run_direction_buf) do
		if value == "right" then
			x = 1
		elseif value == "left" then
			x = -1
		elseif value == "up" then
			y = 1
		else
			y = -1
		end
	end

	if y ~= 0 and x ~= 0 then
		y = y*0.70710678118654752440
		x = x*0.70710678118654752440
	end

	-- set_player_run_vector(x*velocity, y*velocity)
end

for i = -20.4,20.4,1.7 do
	for j = -20.4,20.4,1.7 do
		add_debug_rectangle(i,j,1.2,1.4)
		add_debug_circle(i,j,0.7)
	end
end

fill_physic_world()
