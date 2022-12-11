defmodule Solution do
  @spec delete_greatest_value(grid :: [[integer]]) :: integer
  def delete_greatest_value(grid) do
    grid
    |> Enum.map(fn row -> Enum.sort(row, &>=/2) end)
    |> Enum.zip()
    |> Enum.map(fn tp ->
      tp
      |> Tuple.to_list()
      |> Enum.max()
    end)
    |> Enum.sum()
  end
end
