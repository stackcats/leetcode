defmodule Solution do
  @spec check_x_matrix(grid :: [[integer]]) :: boolean
  def check_x_matrix(grid) do
    len = Enum.count(grid)

    grid
    |> Enum.with_index()
    |> Enum.reduce_while(true, fn {row, i}, acc ->
      row
      |> Enum.with_index()
      |> Enum.reduce_while(acc, fn {n, j}, acc ->
        cond do
          (i == j || i + j + 1 == len) && n == 0 -> {:halt, false}
          i != j && i + j + 1 != len && n != 0 -> {:halt, false}
          true -> {:cont, true}
        end
      end)
      |> then(fn r ->
        if r do
          {:cont, true}
        else
          {:halt, false}
        end
      end)
    end)
  end
end
