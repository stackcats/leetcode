defmodule Solution do
  defmodule Queue do
    @leaf nil
    def new(), do: @leaf

    defp rank(@leaf), do: 0
    defp rank({r, _, _, _}), do: r

    defp merge(@leaf, t), do: t
    defp merge(t, @leaf), do: t

    defp merge({_, lv, ll, lr} = t1, {_, rv, rl, rr} = t2) do
      if lv < rv do
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

    def peek(@leaf), do: nil
    def peek({_, v, _, _}), do: v

    def pop(@leaf), do: nil
    def pop({_, v, @leaf, @leaf}), do: {v, @leaf}
    def pop({_, v, l, r}), do: {v, merge(l, r)}
  end

  @spec min_cost_connect_points(points :: [[integer]]) :: integer
  def min_cost_connect_points(points) do
    {pool, points} =
      points
      |> Enum.with_index()
      |> Enum.reduce({MapSet.new(), %{}}, fn {p, i}, {pool, points} ->
        {MapSet.put(pool, i), Map.put(points, i, p)}
      end)

    pq = Queue.new() |> Queue.push({0, 0})
    aux(pq, pool, points, 0)
  end

  def aux(pq, pool, points, ans) do
    if MapSet.size(pool) == 0 do
      ans
    else
      {{d, to}, pq} = Queue.pop(pq)

      if not MapSet.member?(pool, to) do
        aux(pq, pool, points, ans)
      else
        pool = MapSet.delete(pool, to)

        pool
        |> Enum.reduce(pq, fn i, acc ->
          d = manhattan(points[to], points[i])
          Queue.push(acc, {d, i})
        end)
        |> aux(pool, points, ans + d)
      end
    end
  end

  def manhattan([x1, y1], [x2, y2]) do
    abs(x1 - x2) + abs(y1 - y2)
  end
end
