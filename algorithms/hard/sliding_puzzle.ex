defmodule Solution do
  @spec sliding_puzzle(board :: [[integer]]) :: integer
  def sliding_puzzle(board) do
    g = to_map(board)
    target = to_map([[1, 2, 3], [4, 5, 0]])

    [{g, 0}]
    |> :queue.from_list()
    |> bfs(target, %{})
  end

  def bfs(q, target, seen) do
    case :queue.out(q) do
      {:empty, _} ->
        -1

      {{_, {g, ct}}, q} ->
        cond do
          g == target ->
            ct

          Map.get(seen, g, ct + 10) < ct ->
            bfs(q, target, seen)

          true ->
            seen = Map.put(seen, g, ct)
            {curr = {x, y}, _} = Enum.find(g, fn {_, v} -> v == 0 end)

            [{-1, 0}, {0, 1}, {1, 0}, {0, -1}]
            |> Enum.reduce(q, fn {dx, dy}, q ->
              next = {x + dx, y + dy}

              case g[next] do
                nil ->
                  q

                n ->
                  g =
                    g
                    |> Map.put(curr, n)
                    |> Map.put(next, 0)

                  :queue.in({g, ct + 1}, q)
              end
            end)
            |> bfs(target, seen)
        end
    end
  end

  def to_map(mat) do
    mat
    |> Enum.with_index()
    |> Enum.reduce(%{}, fn {r, i}, acc ->
      r
      |> Enum.with_index()
      |> Enum.reduce(acc, fn {c, j}, acc ->
        Map.put(acc, {i, j}, c)
      end)
    end)
  end
end
