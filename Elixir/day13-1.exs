defmodule PacketScanners do
  def strtonumarray(numlist) do
    trimmedstring=String.trim(numlist)
    endlgone=String.split(trimmedstring,"\n")
    listofstrings=Enum.map(endlgone, fn(x)->String.split(x, ":") end)
    rowsofnums=Enum.map(listofstrings, fn(x)->Enum.map(x, fn(y)->String.to_integer(String.trim(y)) end) end)
    rowsoftuples=Enum.map(rowsofnums, fn(x)->List.to_tuple(x) end)
    rowsoftuples
  end

  def insert_prev_no(decrementor, scanner, prev_no) when decrementor>0 do
    scanner=Map.put_new(scanner, prev_no, [{0,0}])
    insert_prev_no(decrementor-1, scanner, prev_no+1)
  end

  def insert_prev_no(decrementor,scanner,_) when decrementor<=0 do
    scanner
  end

  def traverse_scanner(scanner,[n|range],state,severity) when state==="forward" do



  end

  def create_initial_scan([n|numlist], scanner, prev_no) do
    scanner=if(elem(n,0)>prev_no) do insert_prev_no(elem(n,0)-prev_no,scanner,prev_no) else scanner end
    prev_no=if(elem(n,0)>prev_no) do elem(n,0) else prev_no end

    scanner=Map.put_new(scanner,elem(n,0),List.replace_at(List.duplicate({0,0},elem(n,1)),0,{"S", 0}))
    create_initial_scan(numlist,scanner, prev_no+1)
  end

  def create_initial_scan([], scanner, _) do
    scanner
  end

  def do_traversal() do
    sheet= File.read!("input13.txt")
    numarray=strtonumarray(sheet)
    scanner=create_initial_scan(numarray,%{}, 0)
    range=elem(List.first(numarray),0)..elem(List.last(numarray),0)|>Enum.to_list()
    traverse_scanner(scanner,range,"forward",0)
  end
end
