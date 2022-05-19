defmodule Solution do
  @spec longest_increasing_path(matrix :: [[integer]]) :: integer
  def longest_increasing_path(matrix) do
    r = Enum.count(matrix)
    c = Enum.count(hd(matrix))

    {g, dp} =
      matrix
      |> Enum.with_index()
      |> Enum.reduce({%{}, %{}}, fn {row, i}, acc ->
        row
        |> Enum.with_index()
        |> Enum.reduce(acc, fn {n, j}, {g, dp} ->
          {Map.put(g, {i, j}, n), Map.put(dp, {i, j}, 0)}
        end)
      end)

    for i <- 0..(r - 1), j <- 0..(c - 1), reduce: {0, dp} do
      {ans, dp} ->
        dp = dfs(g, dp, i, j)
        ans = max(ans, dp[{i, j}])
        {ans, dp}
    end
    |> elem(0)
  end

  def dfs(g, dp, x, y) do
    if dp[{x, y}] != 0 do
      dp
    else
      for {dx, dy} <- [{-1, 0}, {0, 1}, {1, 0}, {0, -1}], reduce: dp do
        dp ->
          nx = x + dx
          ny = y + dy

          if Map.get(g, {nx, ny}, -1) > g[{x, y}] do
            dp = dfs(g, dp, nx, ny)
            Map.put(dp, {x, y}, max(dp[{x, y}], dp[{nx, ny}]))
          else
            dp
          end
      end
      |> Map.update!({x, y}, &(&1 + 1))
    end
  end
end
