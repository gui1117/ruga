function rect(func,x,y,width,height)
	for i = x,x+height-1 do
		func(i,y)
		func(i,y+height-1)
	end
	for j = y,y+width-1 do
		func(x,j)
		func(x+width-1,j)
	end
end

function vline(func,x,y,height)
	for j = y,y+height-1 do
		func(x,j)
	end
end

function hline(func,x,y,width)
	for i = x,x+width-1 do
		func(i,y)
	end
end
