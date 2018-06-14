defmodule Duet do
  def spawn_processes() do
    Process.register(spawn(fn->sing_duet() end), :duet0)
    Process.register(spawn(fn->sing_duet() end), :duet1)
  end

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

  def do_commands(command_list, incrementor, max, send_increment, duet_map) when incrementor<max do
    command=Enum.at(command_list,incrementor)
    is_numeric=is_numeric(elem(command,2))
    executed_command=do_command(command,incrementor, send_increment,duet_map,is_numeric)
    incrementor=elem(executed_command,0)
    duet_map=elem(executed_command,1)
    send_increment=if(elem(command,0)=="snd") do {elem(send_increment,0),elem(executed_command,2)} else send_increment end
    do_commands(command_list,incrementor,max,send_increment,duet_map)
  end

  def do_commands(_, incrementor, max, _, duet_map) when incrementor>=max do
    duet_map
  end

  def do_command({command,x,y},incrementor,_ ,duet_map, is_numeric) when command=="set" and is_numeric do
     y=String.to_integer(y)
     duet_map=Map.put(duet_map,x,y)
     {incrementor+1,duet_map}
  end

  def do_command({command,x,y},incrementor,_ ,duet_map, is_numeric) when command=="set" and not is_numeric do
     y=Map.get(duet_map,y)
     duet_map=Map.put(duet_map,x,y)
     {incrementor+1,duet_map}
  end

  def do_command({command,x,y},incrementor,_ ,duet_map, is_numeric) when command=="add" and is_numeric do
     y=String.to_integer(y)
     new_val=Map.get(duet_map,x)+y
     duet_map=Map.put(duet_map,x,new_val)
     {incrementor+1,duet_map}
  end

  def do_command({command,x,y},incrementor,_ ,duet_map, is_numeric) when command=="add" and not is_numeric do
     y=Map.get(duet_map, y)
     new_val=Map.get(duet_map,x)+y
     duet_map=Map.put(duet_map,x,new_val)
     {incrementor+1,duet_map}
  end

  def do_command({command,x,y},incrementor,_ ,duet_map, is_numeric) when command=="mul" and is_numeric do
     y=String.to_integer(y)
     new_val=Map.get(duet_map,x)*y
     duet_map=Map.put(duet_map,x,new_val)
     {incrementor+1,duet_map}
  end

  def do_command({command,x,y},incrementor,_ ,duet_map, is_numeric) when command=="mul" and not is_numeric do
     y=Map.get(duet_map,y)
     new_val=Map.get(duet_map,x)*y
     duet_map=Map.put(duet_map,x,new_val)
     {incrementor+1,duet_map}
  end

  def do_command({command,x,y},incrementor,_ ,duet_map, is_numeric) when command=="mod" and is_numeric do
     y=String.to_integer(y)
     new_val=rem(Map.get(duet_map,x),y)
     duet_map=Map.put(duet_map,x,new_val)
     {incrementor+1,duet_map}
  end

  def do_command({command,x,y},incrementor,_ ,duet_map, is_numeric) when command=="mod" and not is_numeric do
     y=Map.get(duet_map,y)
     new_val=rem(Map.get(duet_map,x),y)
     duet_map=Map.put(duet_map,x,new_val)
     {incrementor+1,duet_map}
  end

  def do_command({command,x,_},incrementor,{_,y} ,duet_map, _) when command=="snd" do
    play_val=Map.get(duet_map,x)
    if(self()==Process.whereis(:duet0)) do
      send Process.whereis(:duet1), {:val1, play_val}
    else
      send Process.whereis(:duet0), {:val2, play_val}
    end

    y=y+1
    {incrementor+1,duet_map,y}
  end

  def do_command({command,x,_},incrementor, {y,z}, duet_map,_ ) when command=="rcv" do
    #y is program id z is number of times sent
     IO.inspect({y,div(z,2)})
     duet_map=if(self()==Process.whereis(:duet0)) do receive do
       {:val2,play_val} -> elem(do_command({"set",x,to_string(play_val)},incrementor,1,duet_map,true), 1)
     end
     else
     receive do
       {:val1,play_val} -> elem(do_command({"set",x,to_string(play_val)},incrementor,1,duet_map,true), 1)
     end
   end
     {incrementor+1,duet_map}
  end

  def do_command({command,x,y},incrementor, _, duet_map, is_numeric) when command=="jgz" and is_numeric do
     jump_val=if(is_numeric(x)) do String.to_integer(x) else Map.get(duet_map,x) end
     y=String.to_integer(y)
     return=if(jump_val>0) do {incrementor+y,duet_map} else {incrementor+1,duet_map} end
     return
  end

  def do_command({command,x,y},incrementor, _, duet_map, is_numeric) when command=="jgz" and not is_numeric do
     jump_val=if(is_numeric(x)) do String.to_integer(x) else Map.get(duet_map,x) end
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
    send_increment=if(self()==Process.whereis(:duet0)) do {0,0} else {1,0} end
    executed_commands=do_commands(command_list,0,Enum.count(command_list),send_increment, duet_map)
    executed_commands
  end
end

Duet.spawn_processes()