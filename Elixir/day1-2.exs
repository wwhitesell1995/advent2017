defmodule HalfNumAdd do
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
    listsize=div(Enum.count(numlist),2)
    indexlist=Enum.with_index(numlist)
    doublelist=Enum.map(indexlist, fn(x) -> Enum.at(numlist, elem(x,1)+(listsize),0) end)
    ziplist=Enum.zip(numlist,doublelist)
    sumlist=Enum.map(ziplist, fn({x, y}) -> addnext(x, y) end)
    total=Enum.sum(sumlist)*2
    total
  end
end
