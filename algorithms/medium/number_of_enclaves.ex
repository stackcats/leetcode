defmodule Solution do
  @spec num_enclaves(grid :: [[integer]]) :: integer
  def num_enclaves(grid) do
    {g, m, n} = to_map(grid)

    g =
      for i <- 0..(m - 1), reduce: g do
        acc -> acc |> dfs(i, 0) |> dfs(i, n - 1)
      end

    for j <- 0..(n - 1), reduce: g do
      acc -> acc |> dfs(0, j) |> dfs(m - 1, j)
    end
    |> Enum.reduce(0, fn {_, v}, acc -> v + acc end)
  end

  def dfs(g, i, j) do
    if g[{i, j}] == 1 do
      g
      |> Map.put({i, j}, 0)
      |> dfs(i + 1, j)
      |> dfs(i - 1, j)
      |> dfs(i, j + 1)
      |> dfs(i, j - 1)
    else
      g
    end
  end

  def to_map(grid) do
    m = Enum.count(grid)
    n = Enum.count(hd(grid))

    grid
    |> Enum.with_index()
    |> Enum.reduce(%{}, fn {row, i}, acc ->
      row
      |> Enum.with_index()
      |> Enum.reduce(acc, fn {n, j}, acc ->
        Map.put(acc, {i, j}, n)
      end)
    end)
    |> then(&{&1, m, n})
  end
end
