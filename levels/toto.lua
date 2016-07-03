add_character(4,4)

for i = 0,100 do
	add_wall(i,0)
	add_wall(i,100)
end
for j = 1,99 do
	add_wall(0,j)
	add_wall(100,j)
end
add_wall(10,10)
add_wall(20,10)
add_wall(20,20)
add_wall(10,20)

add_column(13,10)
add_column(15,10)

add_laser(5,5)

add_monster(40,40)
add_monster(40,40)
add_monster(40,40)
add_monster(40,40)
add_monster(40,40)
add_monster(40,40)
add_monster(40,40)
add_monster(40,40)
add_monster(40,40)

return
