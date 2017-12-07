defmodule AdjacentSquares do

  def move(graph, direction) do
    current=List.first(graph)
    location=List.last(current)
    cond
      direction='r'->move={elem(location,0)+1,elem(location,0)}
      direction='u'->move={elem(location,0),elem(location,0)+1}
      direction='l'->move={elem(location,0)-1,elem(location,0)}
      direction='d'->move={elem(location,0),elem(location,0)-1}
    end
    [[0,move]]
  end

  def sum_neighbors({x,y}, graph) do
    neighbors=Enum.map(neighbors({x, y}), fn(x)->Enum.find())
  end

  def neighbors({x, y}) do
   [{x - 1, y + 1}, {x, y + 1}, {x + 1, y + 1},
   {x - 1, y}, {x + 1, y},
   {x - 1, y - 1}, {x, y - 1}, {x + 1, y - 1}]
  end

  def find_greater(number) do
    start=[{{0,0}, 1}]


  end
end
