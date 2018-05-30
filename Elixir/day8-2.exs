defmodule GetMaxRegisters do

    def comparison(compval, compoperator, compnum) when compoperator === "<" do
        compval < compnum
    end

    def comparison(compval, compoperator, compnum) when compoperator === ">" do
        compval > compnum
    end

    def comparison(compval, compoperator, compnum) when compoperator === "<=" do
        compval <= compnum
    end

    def comparison(compval, compoperator, compnum) when compoperator === ">=" do
        compval >= compnum
    end

    def comparison(compval, compoperator, compnum) when compoperator === "==" do
        compval === compnum
    end

    def comparison(compval, compoperator, compnum) when compoperator === "!=" do
        compval !== compnum
    end

    def calculate(map_value, operator, rvalue) when operator === "inc" do
        map_value+rvalue
    end

    def calculate(map_value, operator, rvalue) when operator === "dec" do
        map_value-rvalue
    end

    def get_max(curr_value, new_value) when new_value>=curr_value do
        new_value
    end

    def get_max(curr_value, new_value) when new_value<curr_value do
        curr_value
    end

    def strtoarray(list) do
        trimmedstring=String.trim(list)
        endlgone=String.split(trimmedstring,"\n")
        listofstrings=Enum.map(endlgone, fn(x)->String.split(x) end)
        listofstrings
    end

    def valuemap(listofstrings) do
        keylist=Enum.map(listofstrings,fn(x)->List.first(x) end)
        uniquelist=Enum.uniq(keylist)
        maplist=Enum.map(uniquelist,fn(x)->%{x=>0} end)
        map=Enum.reduce(maplist,%{},fn(map, acc)->Map.merge(acc, map) end)
        map
    end


    def get_final_value(listofstrings, maplist, increment, size, max_value) when increment<size do

      n=Enum.at(listofstrings,increment)
      calckey=Enum.at(n,0)
      addorsub=Enum.at(n,1)
      numaddorsub=String.to_integer(Enum.at(n,2))
      compkey=Enum.at(n,4)
      compoperator=Enum.at(n,5)
      compnum=String.to_integer(Enum.at(n,6))
      map_value=Map.get(maplist,compkey)
      map_calc=Map.get(maplist,calckey)

      if(comparison(map_value,compoperator,compnum)) do
        new_value=calculate(map_calc, addorsub, numaddorsub)
        max_value=get_max(max_value,new_value)
        new_map=Map.replace!(maplist,calckey,new_value)
        get_final_value(listofstrings, new_map, increment+1, size, max_value)
      else
        get_final_value(listofstrings, maplist, increment+1, size, max_value)
      end
    end

    def get_final_value(_, _, increment, size, max_value) when increment >=size do
      max_value
    end

    def get_commands() do
        sheet= File.read!("input8.txt")
        listofstrings=strtoarray(sheet)
        maplist=valuemap(listofstrings)
        size=Enum.count(listofstrings)
        max_value=get_final_value(listofstrings, maplist, 0, size, 0)|>IO.inspect
        max_value
    end
end
