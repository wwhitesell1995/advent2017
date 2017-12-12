defmodule HexEd do
  def strtoarray(strlist) do
    trimmedstring=String.trim(strlist)
    listofstrings=String.split(trimmedstring,",")
    listofstrings
  end

  def move({x,y,z}, direction) when direction=="n" do
     {x+1,y, z-1}
  end

  def move({x,y,z}, direction) when direction=="s" do
     {x-1,y, z+1}
  end

  def move({x,y,z}, direction) when direction=="nw" do
     {x,y+1, z-1}
  end

  def move({x,y,z}, direction) when direction=="se" do
     {x,y-1, z+1}
  end

  def move({x,y,z}, direction) when direction=="ne" do
     {x+1,y-1,z}
  end

  def move({x,y,z}, direction) when direction=="sw" do
     {x-1,y+1,z}
  end

  def find_distance([n|directions],coordinate) do
    coordinate=move(coordinate, n)
    find_distance(directions,coordinate)
  end

  def find_distance([],coordinate) do
    coordinate
  end

  def make_grid() do
    sheet= File.read!("input11.txt")
    directions=strtoarray(sheet)
    start={0,0,0}
    coordinate_list=Tuple.to_list(find_distance(directions,start))
    positive_list=Enum.map(coordinate_list, fn(x)->abs(x) end)
    distance=Enum.max(positive_list)
    distance
  end
end
