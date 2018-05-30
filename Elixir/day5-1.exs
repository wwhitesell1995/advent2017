defmodule TwistyMaze do
  def strtonumarray(numlist) do
    trimmedstring=String.trim(numlist)
    endlgone=String.split(trimmedstring,"\n")
    rowofnums=Enum.map(endlgone, fn(x)->String.to_integer(x) end)
    rowofnums
  end

  def jump_around(numlist, jump_no, incrementor, list_size) when jump_no>=0 and jump_no<list_size do
      get_num=Enum.at(numlist,jump_no)
      new_numlist=List.update_at(numlist,jump_no,fn(x)->x+1 end)
      jump_around(new_numlist,jump_no+get_num,incrementor+1,list_size)
  end

  def jump_around(_, jump_no, incrementor, list_size) when jump_no<0 or jump_no>=list_size do
    incrementor
  end

  def get_jumps() do
      sheet= File.read!("input5.txt")
      numlist=strtonumarray(sheet)
      list_size=Enum.count(numlist)
      jump_around(numlist,0,0,list_size)
  end
end
