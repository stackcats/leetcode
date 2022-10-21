defmodule Solution do
  @spec answer_queries(nums :: [integer], queries :: [integer]) :: [integer]
  def answer_queries(nums, queries) do
    nums
    |> Enum.sort()
    |> Enum.reduce({[], 0}, fn n, {acc, pre} ->
      {[pre + n | acc], pre + n}
    end)
    |> elem(0)
    |> Enum.reverse()
    |> List.to_tuple()
    |> then(fn nums ->
      for q <- queries do
        search(nums, q)
      end
    end)
  end

  def search(tps, q) do
    search(tps, 0, tuple_size(tps) - 1, q)
  end

  def search(tps, l, r, q) when l > r, do: l

  def search(tps, l, r, q) do
    m = div(l + r, 2)

    if elem(tps, m) <= q do
      search(tps, m + 1, r, q)
    else
      search(tps, l, m - 1, q)
    end
  end
end
