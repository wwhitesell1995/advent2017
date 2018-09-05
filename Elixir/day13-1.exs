defmodule PacketScanners do
  def strtonumarray(numlist) do
    trimmedstring=String.trim(numlist)
    endlgone=String.split(trimmedstring,"\n")
    listofstrings=Enum.map(endlgone, fn(x)->String.split(x, ":") end)
    rowsofnums=Enum.map(listofstrings, fn(x)->Enum.map(x, fn(y)->String.to_integer(String.trim(y)) end) end)
    rowsoftuples=Enum.map(rowsofnums, fn(x)->List.to_tuple(x) end)
    rowsoftuples
  end

  def traverse_scanner([n|range], prev_n, severity, curr_severity) when curr_severity==0 do
    severity=severity+elem(prev_n,0)*elem(prev_n,1)
    curr_severity=rem(elem(n, 0), (2*(elem(n,1)-1)))
    traverse_scanner(range, n, severity, curr_severity)
  end

  def traverse_scanner([n|range], _prev_n, severity, curr_severity) when curr_severity>0 do
    curr_severity=rem(elem(n, 0), (2*(elem(n,1)-1)))
    traverse_scanner(range, n, severity, curr_severity)
  end

  def traverse_scanner([], _, severity, _) do
    severity 
  end

  def do_traversal() do
    sheet= File.read!("input13.txt")
    numarray=strtonumarray(sheet)
    traverse_scanner(numarray, {1,1}, 0, 1)
  end
end
