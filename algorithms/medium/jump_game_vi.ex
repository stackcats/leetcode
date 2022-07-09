defmodule Solution do
  defmodule Queue do
    @leaf nil
    def new(), do: @leaf

    defp rank(@leaf), do: 0
    defp rank({r, _, _, _}), do: r

    defp merge(@leaf, t), do: t
    defp merge(t, @leaf), do: t

    defp merge({_, lv, ll, lr} = t1, {_, rv, rl, rr} = t2) do
      if lv >= rv do
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

  @spec max_result(nums :: [integer], k :: integer) :: integer
  def max_result([n | ns], k) do
    pq = Queue.new() |> Queue.push({n, 0})
    max_result(ns, 1, k, pq, n)
  end

  def max_result([], _i, _k, _pq, ans), do: ans

  def max_result([n | ns], i, k, pq, ans) do
    {top, j} = Queue.peek(pq)

    if j + k < i do
      max_result([n | ns], i, k, Queue.pop(pq) |> elem(1), ans)
    else
      max_result(ns, i + 1, k, Queue.push(pq, {n + top, i}), n + top)
    end
  end
end
