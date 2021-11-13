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

    def peek(@leaf), do: nil
    def peek({_, v, _, _}), do: v

    def pop(@leaf), do: nil
    def pop({_, v, @leaf, @leaf}), do: {v, @leaf}
    def pop({_, v, l, r}), do: {v, merge(l, r)}
  end

  @spec kth_largest_number(nums :: [String.t()], k :: integer) :: String.t()
  def kth_largest_number(nums, k) do
    {l, r} = Enum.split(nums, k)
    q = l |> Enum.reduce(Queue.new(), &Queue.push(&2, String.to_integer(&1)))

    q =
      r
      |> Enum.reduce(q, fn n, q ->
        ns = String.to_integer(n)
        top = Queue.peek(q)

        if ns > top do
          {_, new_q} = Queue.pop(q)
          Queue.push(new_q, ns)
        else
          q
        end
      end)

    "#{Queue.peek(q)}"
  end
end
