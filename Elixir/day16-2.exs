defmodule Prominade do
  def strtoarray(inputlist) do
    trimmedstring=String.trim(inputlist)
    endlgone=String.split(trimmedstring,",")
    listofstrings=Enum.map(endlgone, fn(x)->String.split_at(x, 1) end)
    listofstrings=Enum.map(listofstrings, fn({x,y})->{x,List.to_tuple(String.split(y, "/"))} end)
    listofstrings
  end

  def swap(letter_list,x,y) do
    first_val=Enum.at(letter_list,x)
    second_val=Enum.at(letter_list,y)

    letter_list=List.replace_at(letter_list,x,second_val)
    letter_list=List.replace_at(letter_list,y,first_val)

    letter_list
  end

  def dance([{command,{x}}|command_list], letter_list) when command=="s" do
    x=String.to_integer(x)
    move_tuple=Enum.split(letter_list,Enum.count(letter_list)-x)
    letter_list=elem(move_tuple,1)++elem(move_tuple,0)
    dance(command_list, letter_list)
  end

  def dance([{command,{x,y}}|command_list], letter_list) when command=="x" do
    x=String.to_integer(x)
    y=String.to_integer(y)

    letter_list=swap(letter_list,x,y)

    dance(command_list, letter_list)
  end

  def dance([{command,{x,y}}|command_list], letter_list) when command=="p" do
    index_1=Enum.find_index(letter_list,fn(a)->a==x end)
    index_2=Enum.find_index(letter_list,fn(b)->b==y end)

    letter_list=swap(letter_list,index_1,index_2)

    dance(command_list, letter_list)
  end

  def dance([], letter_list) do
    letter_list
  end

  def no_dances(command_list, letter_list, first_list, incrementor) when letter_list != first_list or incrementor==0 do
    letter_list=dance(command_list, letter_list)
    no_dances(command_list, letter_list, first_list, incrementor+1)
  end

  def no_dances(_, letter_list, first_list, incrementor) when letter_list == first_list and incrementor>0 do
    incrementor
  end

  def dances(command_list, letter_list, incrementor, no_dances) when incrementor != rem(1000000000,no_dances) do
     letter_list=dance(command_list, letter_list)
     dances(command_list, letter_list, incrementor+1, no_dances)
  end

  def dances(_, letter_list, incrementor, no_dances) when incrementor == rem(1000000000,no_dances) do
    letter_list
  end

  def do_dance() do
    sheet= File.read!("input16.txt")
    command_list= strtoarray(sheet)
    letter_list=for n <- ?a..?p, do: << n :: utf8 >>
    no_of_iterations=no_dances(command_list, letter_list, letter_list, 0)
    new_string=Enum.join(dances(command_list,letter_list,0,no_of_iterations))
    new_string
  end
end
