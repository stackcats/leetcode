defmodule Solution do
  defmodule Queue do
    @leaf nil
    def new(), do: @leaf

    defp rank(@leaf), do: 0
    defp rank({r, _, _, _}), do: r

    defp merge(@leaf, t), do: t
    defp merge(t, @leaf), do: t

    defp merge({_, lv, ll, lr} = t1, {_, rv, rl, rr} = t2) do
      if lv <= rv do
        swipe(lv, ll, merge(lr, t2))
      else
        swipe(rv, rl, merge(t1, rr))
      end
    end

    defp swipe(v, left, right) do
      if rank(left) >= rank(right) do
        {rank(right) + 1, v, left, right}
      else
        {rank(left) + 1, v, right, left}
      end
    end

    def push(q, v) do
      merge(q, {1, v, @leaf, @leaf})
    end

    def pop(@leaf), do: nil
    def pop({_, v, @leaf, @leaf}), do: {v, @leaf}
    def pop({_, v, l, r}), do: {v, merge(l, r)}
  end

  @spec network_delay_time(times :: [[integer]], n :: integer, k :: integer) :: integer
  def network_delay_time(times, n, k) do
    q = Queue.new() |> Queue.push({0, k})
    visited = MapSet.new(1..n)

    map =
      times
      |> Enum.reduce(%{}, fn [u, v, w], acc ->
        Map.update(acc, u, [{v, w}], &[{v, w} | &1])
      end)

    bfs(q, visited, map, 0)
  end

  def bfs(nil, visited, _map, ans) do
    if MapSet.size(visited) == 0 do
      ans
    else
      -1
    end
  end

  def bfs(q, visited, map, ans) do
    {{t, s}, q} = Queue.pop(q)

    if not MapSet.member?(visited, s) do
      bfs(q, visited, map, ans)
    else
      q =
        Map.get(map, s, [])
        |> Enum.reduce(q, fn {v, w}, q ->
          Queue.push(q, {w + t, v})
        end)

      bfs(q, MapSet.delete(visited, s), map, max(ans, t))
    end
  end
end
