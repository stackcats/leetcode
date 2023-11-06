defmodule Solution do
  @spec find_champion(grid :: [[integer]]) :: integer
  def find_champion(grid) do
    grid
    |> Enum.map(fn row -> Enum.count(row, &(&1 == 1)) end)
    |> Enum.with_index()
    |> Enum.max_by(&elem(&1, 0))
    |> elem(1)
  end
end
