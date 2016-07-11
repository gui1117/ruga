add_character(4,4)

for i = 0,10 do
	add_wall(i,0)
	add_wall(i,10)
end
for j = 1,9 do
	add_wall(0,j)
	add_wall(10,j)
end

add_portal(5,4,"toto")

return
