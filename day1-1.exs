defmodule NextNumAdd do
  def movebeginningtoend(numlist) do
     firstvalue=List.first(numlist)
     newlist=Enum.reverse(numlist)
     newlist=List.insert_at(newlist,0,firstvalue)
     newlist=Enum.reverse(newlist)
     newlist=List.delete_at(newlist,0)
     newlist
  end

  def addnext(num, num), do: num
  def addnext(_, _), do: 0

  def addingduplicates(number) do
    numlist=Integer.digits(number)
    reverselist=movebeginningtoend(numlist)
    ziplist=Enum.zip(numlist,reverselist)
    sumlist=Enum.map(ziplist, fn({x, y}) -> addnext(x, y) end)
    total=Enum.sum(sumlist)
    total
  end
end
