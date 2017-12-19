defmodule Duet do
  def strtoarray(inputlist) do
    trimmedstring=String.trim(inputlist)
    endlgone=String.split(trimmedstring,"\n")
    listofstrings=Enum.map(endlgone, fn(x)->String.split(x," ") end)
    listofstrings=Enum.map(listofstrings, fn(x)->if(Enum.count(x)<3) do List.insert_at(x,2,"0") else x end end)
    listofstrings=Enum.map(listofstrings, fn(x)->List.to_tuple(x) end)
    listofstrings
  end

  def is_numeric(str) do
    case Float.parse(str) do
      {_num, ""} -> true
      _          -> false
    end
  end

  def do_commands(command_list, incrementor, max, duet_map, prev_map) when incrementor<max do
    command=Enum.at(command_list,incrementor)
    is_numeric=is_numeric(elem(command,2))
    executed_command=if(elem(command,0) == "snd" or elem(command,0) == "rcv") do
                     do_command(command,incrementor,duet_map,prev_map)
                     else
                     do_command(command,incrementor,duet_map,is_numeric)
                     end

    incrementor=elem(executed_command,0)|>IO.inspect
    duet_map=elem(executed_command,1)
    prev_map=if(elem(command,0) == "snd" ) do elem(executed_command,2) else prev_map end
    do_commands(command_list,incrementor,max,duet_map,prev_map)
  end

  def do_commands(_, incrementor, max, duet_map, _) when incrementor>=max do
    duet_map
  end

  def do_command({command,x,y},incrementor, duet_map, is_numeric) when command=="set" and is_numeric do
     y=String.to_integer(y)
     duet_map=Map.put(duet_map,x,y)
     {incrementor+1,duet_map}
  end

  def do_command({command,x,y},incrementor, duet_map, is_numeric) when command=="set" and not is_numeric do
     y=Map.get(duet_map,y)
     duet_map=Map.put(duet_map,x,y)
     {incrementor+1,duet_map}
  end

  def do_command({command,x,y},incrementor, duet_map, is_numeric) when command=="add" and is_numeric do
     y=String.to_integer(y)
     new_val=Map.get(duet_map,x)+y
     duet_map=Map.put(duet_map,x,new_val)
     {incrementor+1,duet_map}
  end

  def do_command({command,x,y},incrementor, duet_map, is_numeric) when command=="add" and not is_numeric do
     y=Map.get(duet_map, y)
     new_val=Map.get(duet_map,x)+y
     duet_map=Map.put(duet_map,x,new_val)
     {incrementor+1,duet_map}
  end

  def do_command({command,x,y},incrementor, duet_map, is_numeric) when command=="mul" and is_numeric do
     y=String.to_integer(y)
     new_val=Map.get(duet_map,x)*y
     duet_map=Map.put(duet_map,x,new_val)
     {incrementor+1,duet_map}
  end

  def do_command({command,x,y},incrementor, duet_map, is_numeric) when command=="mul" and not is_numeric do
     y=Map.get(duet_map,y)
     new_val=Map.get(duet_map,x)*y
     duet_map=Map.put(duet_map,x,new_val)
     {incrementor+1,duet_map}
  end

  def do_command({command,x,y},incrementor, duet_map, is_numeric) when command=="mod" and is_numeric do
     y=String.to_integer(y)
     new_val=rem(Map.get(duet_map,x),y)
     duet_map=Map.put(duet_map,x,new_val)
     {incrementor+1,duet_map}
  end

  def do_command({command,x,y},incrementor, duet_map, is_numeric) when command=="mod" and not is_numeric do
     y=Map.get(duet_map,y)
     new_val=rem(Map.get(duet_map,x),y)
     duet_map=Map.put(duet_map,x,new_val)
     {incrementor+1,duet_map}
  end

  def do_command({command,x,_},incrementor, duet_map, prev_map) when command=="snd" do
     play_val=Map.get(duet_map,x)
     {incrementor+1,duet_map,Map.put(prev_map,x,play_val)}
  end

  def do_command({command,x,_},incrementor, duet_map, prev_map) when command=="rcv" do
     play_val=Map.get(prev_map,x)
     if (play_val>0) do
       #IO.puts(play_val)
      {incrementor+1,duet_map}
     else
      {incrementor+1,duet_map}
     end
  end

  def do_command({command,x,y},incrementor, duet_map, is_numeric) when command=="jgz" and is_numeric do
     jump_val=Map.get(duet_map,x)
     y=String.to_integer(y)
     return=if(jump_val>0) do {incrementor+y,duet_map} else {incrementor+1,duet_map} end
     return
  end

  def do_command({command,x,y},incrementor, duet_map, is_numeric) when command=="jgz" and not is_numeric do
     jump_val=Map.get(duet_map,x)
     new_jump=Map.get(duet_map,y)
     return=if(jump_val>0) do
            {incrementor+new_jump,duet_map}
            else
            {incrementor+1,duet_map}
            end
     return
  end

  def create_register(command_list) do
    registers=Enum.map(command_list,fn({_,y,_})->y end)
    registers=Enum.uniq(registers)
    registers=Enum.map(registers,fn(x)->{x,0} end)
    duet_map=Map.new(registers)
    duet_map
  end

  def sing_duet() do
    sheet= File.read!("input18.txt")
    command_list=strtoarray(sheet)
    duet_map=create_register(command_list)
    executed_commands=do_commands(command_list,0,Enum.count(command_list),duet_map,%{})
    executed_commands
  end
end
