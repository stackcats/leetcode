defmodule Solution do
  @spec number_game(nums :: [integer]) :: [integer]
  def number_game(nums) do
    nums
    |> Enum.sort()
    |> Enum.chunk_every(2)
    |> Enum.flat_map(fn [a, b] -> [b, a] end)
  end
end
