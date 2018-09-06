defmodule PacketScanners do
  def strtonumarray(numlist) do
    trimmedstring=String.trim(numlist)
    endlgone=String.split(trimmedstring,"\n")
    listofstrings=Enum.map(endlgone, fn(x)->String.split(x, ":") end)
    rowsofnums=Enum.map(listofstrings, fn(x)->Enum.map(x, fn(y)->String.to_integer(String.trim(y)) end) end)
    rowsoftuples=Enum.map(rowsofnums, fn(x)->List.to_tuple(x) end)
    rowsoftuples
  end

  def get_min_picoseconds(numarray, num_seconds) do
     is_caught=Enum.any?(numarray, fn {x,y} -> rem(num_seconds+x, 2*(y-1))==0 end)
     case is_caught do
      true -> get_min_picoseconds(numarray, num_seconds + 1)
      false ->  num_seconds
     end
  end
  
  def do_traversal() do
    sheet= File.read!("input13.txt")
    numarray=strtonumarray(sheet)
    get_min_picoseconds(numarray, 0)
  end
end
