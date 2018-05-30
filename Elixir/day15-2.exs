defmodule DuelingGenerators do
  @start_a 618
  @start_b 814
  @a_factor 16807
  @b_factor 48271
  @div 2147483647
  @count 5000000

  def score({a, b}, n) do
      a_list=a
      |> nexta()
      |> Stream.iterate(&nexta/1)
      |> Stream.filter(fn(x)->rem(x,4)==0 end)
      |> Stream.take(n)
      |> Stream.map(&lower_16/1)
      |> Enum.to_list()


      b_list=b
      |> nextb()
      |> Stream.iterate(&nextb/1)
      |> Stream.filter(fn(x)->rem(x,8)==0 end)
      |> Stream.take(n)
      |> Stream.map(&lower_16/1)
      |> Enum.to_list()

      final_list=make_tuple_list(a_list,b_list,[])

      Enum.count(final_list, fn {a, b} -> a == b end)
    end

  defp make_tuple_list([a|list_a], [b|list_b], final_list) do
    final_list=[{a,b}|final_list]
    make_tuple_list(list_a,list_b,final_list)
  end

  defp make_tuple_list([],[],final_list) do
    final_list
  end

  defp nexta(a) do
    next_a = rem(a * @a_factor, @div)
    next_a
  end

  defp nextb(b) do
    next_b = rem(b * @b_factor, @div)
    next_b
  end

  defp lower_16(x) do
    lower_n(x, 16)
  end


    defp lower_n(int, n) do
      <<int::little-size(n)>>
    end


    def get_judge_count() do
      start_pair={@start_a*@a_factor, @start_b*@b_factor}
      judge_count=score(start_pair, @count)
      judge_count
    end
end
