defmodule BlockInBanks do
  def strtonumarray(numlist) do
    trimmedstring=String.trim(numlist)
    endlgone=String.split(trimmedstring,"\n")
    listofstrings=Enum.map(endlgone, fn(x)->String.split(x) end)
    rowsofnums=Enum.map(listofstrings, fn(x)->Enum.map(x, fn(y)->String.to_integer(y) end) end)
    rowsofnums
  end

  def distribute(blocks, curr_index, max_block, incrementor) when incrementor<max_block do
      blocks_size=Enum.count(blocks)
      update_index=Integer.mod(blocks_size+Integer.mod(curr_index, blocks_size),blocks_size)
      updated_blocks=List.update_at(blocks, update_index, fn(x)->x+1 end)
      distribute(updated_blocks, curr_index+1, max_block, incrementor+1)
  end

  def distribute(blocks, _, max_block, incrementor) when incrementor>=max_block do
    blocks
  end

  def update_max(blocks) do
    max_block=Enum.max(blocks)
    max_index=Enum.find_index(blocks, fn(x)->x===max_block end)
    updated_blocks=List.update_at(blocks,max_index,fn(_)-> 0 end)
    updated_blocks=distribute(updated_blocks, max_index+1, max_block, 0)
    updated_blocks
  end

  def same_blocks(blocks, incrementor) do
    first_blocks=List.first(blocks)
    updated_blocks=update_max(first_blocks)
    if(Enum.find(blocks,fn(x)->x===updated_blocks end)) do
      blocks=Enum.with_index(blocks)
      {blocks, updated_blocks}
    else
      blocks=List.insert_at(blocks,0,updated_blocks)
      same_blocks(blocks,incrementor+1)
    end
  end

  def find_cycle_until_same() do
    sheet= File.read!("input6.txt")
    numarray=strtonumarray(sheet)
    num_blocks=same_blocks(numarray, 1)
    blocks=elem(num_blocks,0)
    updated_blocks=elem(num_blocks,1)
    num_cycles=Enum.find(blocks,fn(x)->elem(x,0)===updated_blocks end)
    num_iterations=elem(num_cycles,1)+1
    num_iterations
  end
end
