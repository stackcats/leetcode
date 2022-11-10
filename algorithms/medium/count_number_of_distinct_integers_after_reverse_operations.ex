defmodule Solution do
  @spec count_distinct_integers(nums :: [integer]) :: integer
  def count_distinct_integers(nums) do
    nums
    |> Enum.reduce(MapSet.new(nums), fn n, set ->
      MapSet.put(set, re(n))
    end)
    |> MapSet.size()
  end

  def re(n, m \\ 0)
  def re(0, m), do: m

  def re(n, m) do
    re(div(n, 10), m * 10 + rem(n, 10))
  end
end
