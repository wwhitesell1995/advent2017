defmodule Spinlock do
  def iterate(input,numlist,prev_val,incrementor) when incrementor<2018 do
     prev_val=rem(prev_val+input,Enum.count(numlist))+1
     numlist=List.insert_at(numlist,prev_val,incrementor)
     iterate(input,numlist,prev_val,incrementor+1)
  end

  def iterate(_,numlist,prev_val,incrementor) when incrementor>=2018 do
     {numlist,prev_val}
  end


  def complete_buffer() do
    input=367
    start_list=[0]
    prev_val=0
    iterate_values=iterate(input,start_list,prev_val,1)
    numlist=elem(iterate_values,0)
    final_index=elem(iterate_values,1)
    Enum.at(numlist,final_index+1)
  end
end
