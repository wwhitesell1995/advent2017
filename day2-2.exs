defmodule EvenDivisible do
  def sortedstrtonumarray(numlist) do
    trimmedstring=String.trim(numlist)
    endlgone=String.split(trimmedstring,"\n")
    listofstrings=Enum.map(endlgone, fn(x)->String.split(x) end)
    rowsofnums=Enum.map(listofstrings, fn(x)->Enum.map(x, fn(y)->String.to_integer(y) end) end)
    sortednums=Enum.map(rowsofnums, fn(x)->Enum.reverse(Enum.sort(x)) end) |> IO.inspect()
    sortednums
  end

  def divisible(num1, num2) when rem(num1,num2) === 0 and num1 !== num2 do
    true
  end

  def divisible(x, y) when rem(x,y) !== 0 or x === y do
    false
  end

  def matching([n|list]) do
    val=Enum.find(list,fn(x)->divisible(n,x) end)
    if(val) do
      div(n, val)
    else
      matching(list)
    end
  end


  def evenlydivisible() do
    sheet= File.read!("input2_2.txt")
    numlist=sortedstrtonumarray(sheet)
    divisiblelist=Enum.map(numlist, fn(x)-> matching(x) end)
    total=Enum.sum(divisiblelist)
    total
  end
end
