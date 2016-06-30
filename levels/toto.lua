add_character(10,10)

for i = 0,100 do
	add_wall(i,0)
	add_wall(i,100)
end
for j = 1,99 do
	add_wall(0,j)
	add_wall(100,j)
end
add_wall(10,10)

-- add_column(13,10)

add_laser(5,5)

add_monster(5,10)

return
