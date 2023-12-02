defmodule Solution do
  @spec max_div_score(nums :: [integer], divisors :: [integer]) :: integer
  def max_div_score(nums, divisors) do
    divisors
    |> Enum.map(fn d ->
      {Enum.count(nums, &(rem(&1, d) == 0)), d}
    end)
    |> Enum.max_by(fn {cnt, d} -> {cnt, -d} end)
    |> elem(1)
  end
end
