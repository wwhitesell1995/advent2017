defmodule ManhattanDistance do
   def nearest_square(number, square) when (square*square)>=number do
     {square, square*square}
   end

   def nearest_square(number, square) when (square*square)<number do
     nearest_square(number, square+2)
   end

   def find_distance(square, number) do
     max_distance=elem(square,0)-1
     min_distance=div(max_distance,2)
     max_no=elem(square,1)
     corners=[max_no-(max_distance*3),max_no-(max_distance*2),max_no-max_distance,max_no]
     mid_points=Enum.map(corners, fn(x)->{x,x-(min_distance)} end )
     side=Enum.find(mid_points, fn({x,_})->x>=number end)
     distance=min_distance+abs(number-elem(side,1))
     distance
   end

   def taxicab(number) do
     square=nearest_square(number, 1)
     find_distance(square,number)
   end
end
