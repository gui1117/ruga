add_character(0,0)

rect(add_wall,-1,-1,3,3)
add_laser(1,0)

rect(add_wall,-4,-4,9,9)
vline(add_laser,4,-1,3)

hline(add_wall,-4,-4,34)
hline(add_wall,-4,4,34)

vline(add_wall,30,4,3)
vline(add_wall,34,4,3)
vline(add_wall,30,-6,3)
vline(add_wall,34,-6,3)
hline(add_wall,30,7,5)
hline(add_wall,30,-6,5)
add_monster(32,5)
add_monster(32,-5)

hline(add_wall,34,-4,32)
hline(add_wall,34,4,32)
vline(add_laser,45,-3,7)

vline(add_wall,65,4,3)
vline(add_wall,69,4,3)
vline(add_wall,65,-6,3)
vline(add_wall,69,-6,3)
hline(add_wall,65,7,5)
hline(add_wall,65,-6,5)
add_monster(67,5)
add_monster(67,-5)
