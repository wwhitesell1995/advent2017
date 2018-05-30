defmodule Spinlock do
  def iterate(input,numlist,prev_val,incrementor) when incrementor<50000000 do
     prev_val=rem(prev_val+input,incrementor)+1
     numlist=if(prev_val==1) do incrementor else numlist end
     iterate(input,numlist,prev_val,incrementor+1)
  end

  def iterate(_,numlist,_,incrementor) when incrementor>=50000000 do
      numlist
  end


  def complete_buffer() do
    input=367
    start_list=[0]
    prev_val=0
    iterate_values=iterate(input,start_list,prev_val,1)
    iterate_values
  end
end
