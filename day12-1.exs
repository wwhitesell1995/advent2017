defmodule DigitalPlumber do
  def strtonumarray(numlist) do
    trimmedstring=String.trim(numlist)
    endlgone=String.split(trimmedstring,"\n")
    listofstrings=Enum.map(endlgone, fn(x)->String.split(x, ["<->",","]) end)
    rowsofnums=Enum.map(listofstrings, fn(x)->Enum.map(x, fn(y)->String.to_integer(String.trim(y)) end) end)
    rowsofnums
  end

  def list_to_map([n|list]) do
    %{n=>list}
  end

  def merge_maps([n|list], map) do
    map=Map.merge(map,n)
    merge_maps(list,map)
  end

  def merge_maps([], map) do
    map
  end

  def get_group([n|listofnums], numlist, nummap, already_been) do
    if(Enum.find(already_been, fn(x)->x===n end)) do
      get_group(listofnums,numlist,nummap, already_been)
    else
      curr_list=Map.get(nummap, n)|>IO.inspect
      already_been=[n|already_been]
      listofnums=if (curr_list) do listofnums++curr_list else listofnums end
      numlist=if (curr_list) do numlist++curr_list else numlist end
      numlist=Enum.uniq(numlist)
      get_group(listofnums, numlist, nummap, already_been)
    end
  end

  def get_group([],numlist,_,_) do
    numlist
  end

  def do_the_plumbing() do
      sheet= File.read!("input12.txt")
      numarray=strtonumarray(sheet)
      startlist=[0]
      maplist=Enum.map(numarray, fn(x)->list_to_map(x) end)
      map=%{}
      map=merge_maps(maplist, map)
      numgroup=get_group(startlist,startlist,map,[])
      groupsize=Enum.count(numgroup)
      groupsize
  end
end
