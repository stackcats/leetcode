defmodule Solution do
  @spec minimum_cost(cost :: [integer]) :: integer
  def minimum_cost(cost) do
    cost
    |> Enum.sort(&>=/2)
    |> Enum.chunk_every(3)
    |> Enum.map(fn chunk -> chunk |> Enum.take(2) |> Enum.sum() end)
    |> Enum.sum()
  end
end
