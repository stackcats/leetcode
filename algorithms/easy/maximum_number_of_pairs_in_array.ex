defmodule Solution do
  @spec number_of_pairs(nums :: [integer]) :: [integer]
  def number_of_pairs(nums) do
    nums
    |> Enum.reduce(%{}, fn n, acc ->
      Map.update(acc, n, 1, &(&1 + 1))
    end)
    |> Enum.reduce([0, 0], fn {_, v}, [a, b] ->
      [a + div(v, 2), b + rem(v, 2)]
    end)
  end
end
