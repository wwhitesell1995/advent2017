defmodule AdjacentSquares do

  def move(current, direction) do
    location=elem(current,0)
    x=elem(location,0)
    y=elem(location,1)
    cond do
      direction==="r"->{x+1,y}
      direction==="u"->{x,y+1}
      direction==="l"->{x-1,y}
      direction==="d"->{x,y-1}
    end
  end

  def sum_neighbors(graph, decrementor, direction) when decrementor>0 do
    current=List.first(graph)
    move=move(current, direction)|>IO.inspect
    neighbors=neighbors(move)
    adjacent_values=Enum.map(neighbors, fn(x)->elem(List.keyfind(graph,x,0,{x, 0}),1) end)
    sum_values=Enum.sum(adjacent_values)
    graph=[{move, sum_values}]++graph
    sum_neighbors(graph, decrementor-1, direction)
  end

  def sum_neighbors(graph, decrementor, _) when decrementor<=0 do
    graph
  end

  def iterate(directions, curr_value, incrementor, graph, input) when curr_value<=input do
    direction=Enum.at(directions, rem(incrementor-1,4))|>IO.inspect
    graph=sum_neighbors(graph, round(incrementor/2), direction)
    iterate(directions, elem(List.first(graph),1), incrementor+1, graph, input)
  end

  def iterate(_, curr_value, _, graph, input) when curr_value>input do
    graph
  end

  def neighbors({x, y}) do
   [{x - 1, y + 1}, {x, y + 1}, {x + 1, y + 1},
   {x - 1, y}, {x + 1, y},
   {x - 1, y - 1}, {x, y - 1}, {x + 1, y - 1}]
  end

  def find_greater() do
    start=[{{0,0},1}]
    directions=["r","u","l","d"]
    input=265149
    graph=iterate(directions, elem(List.first(start),1), 1, start, input)
    graph
  end
end
