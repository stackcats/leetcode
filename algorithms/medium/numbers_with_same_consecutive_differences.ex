defmodule Solution do
  @spec nums_same_consec_diff(n :: integer, k :: integer) :: [integer]
  def nums_same_consec_diff(n, k) do
    1..9
    |> Enum.map(fn i -> {i, 1} end)
    |> :queue.from_list()
    |> bfs(n, k, [])
  end

  def bfs(q, n, k, ans) do
    if :queue.is_empty(q) do
      ans
    else
      {x, ct} = :queue.head(q)
      q = :queue.tail(q)

      if ct == n do
        bfs(q, n, k, [x | ans])
      else
        d = rem(x, 10)
        q = if d + k <= 9, do: :queue.in({x * 10 + d + k, ct + 1}, q), else: q
        q = if k != 0 && d - k >= 0, do: :queue.in({x * 10 + d - k, ct + 1}, q), else: q
        bfs(q, n, k, ans)
      end
    end
  end
end
