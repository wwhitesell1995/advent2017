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

  def find_max_distance([n|directions],{x,y,z}, x_list, y_list, z_list) do
    {x,y,z}=move({x,y,z}, n)
    x_list=List.insert_at(x_list,0,abs(x))
    y_list=List.insert_at(y_list,0,abs(y))
    z_list=List.insert_at(z_list,0,abs(z))
    find_max_distance(directions,{x,y,z},x_list,y_list,z_list)
  end

  def find_max_distance([],{_,_,_}, x_list, y_list, z_list) do
    max_x=Enum.max(x_list)
    max_y=Enum.max(y_list)
    max_z=Enum.max(z_list)
    [max_x,max_y,max_z]
  end

  def make_grid() do
    sheet= File.read!("input11.txt")
    directions=strtoarray(sheet)
    start={0,0,0}
    max_list=find_max_distance(directions,start,[],[],[])
    max_distance=Enum.max(max_list)
    max_distance
  end
end
