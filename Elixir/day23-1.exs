defmodule Coprocessor do
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

  def do_commands(command_list, incrementor, max, coprocessor_map, prev_map, mul_count) when incrementor<max do
    command=Enum.at(command_list,incrementor)
    is_numeric=is_numeric(elem(command,2))
    executed_command=if(elem(command,0) == "snd" or elem(command,0) == "rcv") do
                     do_command(command,incrementor,coprocessor_map,prev_map, mul_count)
                     else
                     do_command(command,incrementor,coprocessor_map,is_numeric, mul_count)
                     end

    incrementor=elem(executed_command,0)
    coprocessor_map=elem(executed_command,1)
    mul_count=elem(executed_command, 2)
    prev_map=if(elem(command,0) == "snd" ) do elem(executed_command,2) else prev_map end
    do_commands(command_list,incrementor,max,coprocessor_map,prev_map, mul_count)
  end

  def do_commands(_, incrementor, max, _, _, mul_count) when incrementor>=max do
    mul_count
  end

  def do_command({command,x,y},incrementor, coprocessor_map, is_numeric, mul_count) when command=="set" and is_numeric do
     y=String.to_integer(y)
     coprocessor_map=Map.put(coprocessor_map,x,y)
     {incrementor+1,coprocessor_map,mul_count}
  end

  def do_command({command,x,y},incrementor, coprocessor_map, is_numeric, mul_count) when command=="set" and not is_numeric do
     y=Map.get(coprocessor_map,y)
     coprocessor_map=Map.put(coprocessor_map,x,y)
     {incrementor+1,coprocessor_map, mul_count}
  end

  def do_command({command,x,y},incrementor, coprocessor_map, is_numeric, mul_count) when command=="add" and is_numeric do
     y=String.to_integer(y)
     new_val=Map.get(coprocessor_map,x)+y
     coprocessor_map=Map.put(coprocessor_map,x,new_val)
     {incrementor+1,coprocessor_map, mul_count}
  end

  def do_command({command,x,y},incrementor, coprocessor_map, is_numeric, mul_count) when command=="add" and not is_numeric do
     y=Map.get(coprocessor_map, y)
     new_val=Map.get(coprocessor_map,x)+y
     coprocessor_map=Map.put(coprocessor_map,x,new_val)
     {incrementor+1,coprocessor_map, mul_count}
  end

  def do_command({command,x,y},incrementor, coprocessor_map, is_numeric, mul_count) when command=="sub" and is_numeric do
     y=String.to_integer(y)
     new_val=Map.get(coprocessor_map,x)-y
     coprocessor_map=Map.put(coprocessor_map,x,new_val)
     {incrementor+1,coprocessor_map, mul_count}
  end

  def do_command({command,x,y},incrementor, coprocessor_map, is_numeric, mul_count) when command=="sub" and not is_numeric do
     y=Map.get(coprocessor_map, y)
     new_val=Map.get(coprocessor_map,x)-y
     coprocessor_map=Map.put(coprocessor_map,x,new_val)
     {incrementor+1,coprocessor_map, mul_count}
  end

  def do_command({command,x,y},incrementor, coprocessor_map, is_numeric, mul_count) when command=="mul" and is_numeric do
     y=String.to_integer(y)
     new_val=Map.get(coprocessor_map,x)*y
     coprocessor_map=Map.put(coprocessor_map,x,new_val)
     {incrementor+1,coprocessor_map, mul_count+1}
  end

  def do_command({command,x,y},incrementor, coprocessor_map, is_numeric, mul_count) when command=="mul" and not is_numeric do
     y=Map.get(coprocessor_map,y)
     new_val=Map.get(coprocessor_map,x)*y
     coprocessor_map=Map.put(coprocessor_map,x,new_val)
     {incrementor+1,coprocessor_map, mul_count+1}
  end

  def do_command({command,x,y},incrementor, coprocessor_map, is_numeric, mul_count) when command=="mod" and is_numeric do
     y=String.to_integer(y)
     new_val=rem(Map.get(coprocessor_map,x),y)
     coprocessor_map=Map.put(coprocessor_map,x,new_val)
     {incrementor+1,coprocessor_map, mul_count}
  end

  def do_command({command,x,y},incrementor, coprocessor_map, is_numeric, mul_count) when command=="mod" and not is_numeric do
     y=Map.get(coprocessor_map,y)
     new_val=rem(Map.get(coprocessor_map,x),y)
     coprocessor_map=Map.put(coprocessor_map,x,new_val)
     {incrementor+1,coprocessor_map, mul_count}
  end

 #Sound
  def do_command({command,x,_},incrementor, coprocessor_map, prev_map, mul_count) when command=="snd" do
     play_val=Map.get(coprocessor_map,x)
     {incrementor+1,coprocessor_map,Map.put(prev_map,x,play_val), mul_count}
  end

  #Recover
  def do_command({command,x,_},incrementor, coprocessor_map, prev_map, mul_count) when command=="rcv" do
     play_val=Map.get(prev_map,x)
     if (play_val>0) do
       IO.puts(play_val)
      {incrementor+1,coprocessor_map, mul_count}
     else
      {incrementor+1,coprocessor_map, mul_count}
     end
  end

  def do_command({command,x,y},incrementor, coprocessor_map, is_numeric, mul_count) when command=="jnz" and is_numeric do
     jump_val=if(is_numeric(x)) do String.to_integer(x) else Map.get(coprocessor_map,x) end
     y=String.to_integer(y)
     return=if(jump_val !== 0) do {incrementor+y,coprocessor_map, mul_count} else {incrementor+1,coprocessor_map, mul_count} end
     return
  end

  def do_command({command,x,y},incrementor, coprocessor_map, is_numeric, mul_count) when command=="jnz" and not is_numeric do
     jump_val=if(is_numeric(x)) do String.to_integer(x) else Map.get(coprocessor_map,x) end
     new_jump=Map.get(coprocessor_map,y)
     return=if(jump_val !== 0) do
            {incrementor+new_jump,coprocessor_map, mul_count}
            else
            {incrementor+1,coprocessor_map, mul_count}
            end
     return
  end

  def create_register(command_list) do
    registers=Enum.map(command_list,fn({_,y,_})->y end)
    registers=Enum.uniq(registers)
    registers=Enum.map(registers,fn(x)->{x,0} end)
    coprocessor_map=Map.new(registers)
    coprocessor_map
  end

  def debug_coprocessor() do
    sheet= File.read!("input23.txt")
    command_list=strtoarray(sheet)
    coprocessor_map=create_register(command_list)
    executed_commands=do_commands(command_list,0,Enum.count(command_list),coprocessor_map,%{},0)
    executed_commands
  end
end
