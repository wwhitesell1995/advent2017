defmodule DuelingGenerators do
  @start_a 618
  @start_b 814
  @a_factor 16807
  @b_factor 48271
  @div 2147483647
  @count 40000000

  def score({a, b}, n) do
      {a, b}
      |> next()
      |> Stream.iterate(&next/1)
      |> Stream.take(n)
      |> Stream.map(&lower_16/1)
      |> Enum.count(fn {a, b} -> a == b end)
    end

  defp next({a, b}) do
    next_a = rem(a * @a_factor, @div)
    next_b = rem(b * @b_factor, @div)
    {next_a, next_b}
  end

  defp lower_16({a, b}) do
    {lower_n(a, 16), lower_n(b, 16)}
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
