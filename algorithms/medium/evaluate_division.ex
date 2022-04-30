defmodule Solution do
  @spec calc_equation(equations :: [[String.t()]], values :: [float], queries :: [[String.t()]]) ::
          [float]
  def calc_equation(equations, values, queries) do
    acc =
      Enum.zip(equations, values)
      |> Enum.reduce(%{}, fn {[u, v], value}, acc ->
        acc
        |> Map.update(u, [{v, value}], &[{v, value} | &1])
        |> Map.update(v, [{u, 1 / value}], &[{u, 1 / value} | &1])
      end)

    Enum.map(queries, fn [u, v] ->
      if not Map.has_key?(acc, u) or not Map.has_key?(acc, v) do
        -1
      else
        dfs(u, v, MapSet.new(), acc, 1)
      end
    end)
  end

  def dfs(u, u, visited, acc, value), do: value

  def dfs(u, v, visited, acc, value) do
    visited = MapSet.put(visited, u)

    acc[u]
    |> Enum.filter(fn {v, _} ->
      not MapSet.member?(visited, v)
    end)
    |> loop(v, visited, acc, value)
  end

  def loop([], v, visited, acc, value), do: -1

  def loop([{u, val} | rest], v, visited, acc, value) do
    r = dfs(u, v, visited, acc, value * val)
    (r != -1 && r) || loop(rest, v, visited, acc, value)
  end
end
