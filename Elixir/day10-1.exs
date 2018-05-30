defmodule KnotHash do

  def calculate_knot([n|input], numlist, curr_pos, skip_size) do
      IO.inspect(numlist)
      prev_pos=curr_pos|>IO.inspect
      curr_pos=curr_pos+n

      if(n<2) do
        new_pos=curr_pos+skip_size
        new_pos=Integer.mod(256+Integer.mod(new_pos, 256), 256)
        calculate_knot(input, numlist, new_pos, skip_size+1)
      else

      if(curr_pos>=256) do
        new_pos=Integer.mod(256+Integer.mod(curr_pos, 256), 256)|>IO.inspect
        curr_list=if(new_pos<=prev_pos) do Enum.slice(numlist, new_pos..(prev_pos-1)) else Enum.slice(numlist, prev_pos..new_pos) end
        #IO.inspect(curr_list)

        before_list=if(n==256) do
                    Enum.split(numlist, new_pos)
                    else
                    {Enum.slice(numlist, 0..new_pos-1),Enum.slice(numlist, 0..new_pos-1)} end


        after_list=if(n==256) do
                   Enum.split(numlist, new_pos)
                   else
                   {Enum.slice(numlist, prev_pos..Enum.count(numlist)),Enum.slice(numlist, prev_pos..Enum.count(numlist))} end

        reverse_list=Enum.reverse(elem(after_list,1)++elem(before_list,0))
        amount_split=256-prev_pos
        before_and_after=
          if(n==256) do
            {Enum.slice(reverse_list, 0..(Enum.count(reverse_list)-new_pos)-1), Enum.slice(reverse_list, (Enum.count(reverse_list)-new_pos)..Enum.count(reverse_list)) }
          else
            {Enum.slice(reverse_list, (amount_split)..Enum.count(reverse_list)), Enum.slice(reverse_list, 0..amount_split-1) }
          end

          IO.inspect(before_and_after)

        before_list=elem(before_and_after,0)
        after_list=elem(before_and_after,1)
        new_list=if(n==256) do after_list++before_list else before_list++curr_list++after_list end
        new_pos=new_pos+skip_size
        new_pos=Integer.mod(256+Integer.mod(new_pos, 256), 256)
        calculate_knot(input, new_list, new_pos, skip_size+1)
      else
        split_list=Enum.split(numlist, curr_pos)
        reverse_list=Enum.reverse(elem(split_list,0))
        new_list=reverse_list++elem(split_list,1)
        curr_pos=curr_pos+skip_size
        curr_pos=Integer.mod(256+Integer.mod(curr_pos, 256), 256)
        calculate_knot(input, new_list, curr_pos, skip_size+1)
      end
    end
  end

  def calculate_knot([],numlist,_,_) do
    IO.inspect(numlist)
    numlist
  end

  def do_knot() do
     input=[97,167,54,178,2,11,209,174,119,248,254,0,255,1,64,190]
     #input=[3, 4, 1, 5]
     numlist=0..255|>Enum.to_list()
     finalnumlist=calculate_knot(input,numlist,0,0)
     productno=List.first(finalnumlist)*Enum.at(finalnumlist,1)
     productno
  end
end
