defmodule ValidAnagramPasswords do
  def strtoarray(strlist) do
    trimmedstring=String.trim(strlist)
    endlgone=String.split(trimmedstring,"\n")
    listofstrings=Enum.map(endlgone, fn(x)->String.split(x) end)
    listofstrings
  end

  def matching([n|list]) do
    val=Enum.find(list,fn(x)->n===x end)
    if(val) do
      0
    else
      matching(list)
    end
  end

  def matching([]) do
    1
  end

  def sort_passwords(passwords) do
    sorted_passwords=Enum.map(passwords, fn(x)->Enum.map(x, fn(y)->to_string(Enum.sort(String.to_charlist(y))) end) end)
    sorted_passwords
  end

  def find_num_valid() do
    sheet= File.read!("input4.txt")
    passwords=strtoarray(sheet)
    sorted_passwords=sort_passwords(passwords)
    valid_passwords=Enum.map(sorted_passwords, fn(x)->matching(x) end)
    num_valid=Enum.sum(valid_passwords)
    num_valid
  end
end
