defmodule Solution do
  @spec has_all_codes(s :: String.t(), k :: integer) :: boolean
  def has_all_codes(s, k) do
    s
    |> String.graphemes()
    |> aux(:queue.new(), 0, MapSet.new(), k)
    |> MapSet.size()
    |> then(&(&1 == Integer.pow(2, k)))
  end

  def aux([], q, k, set, k) do
    MapSet.put(set, :queue.to_list(q))
  end

  def aux([], _q, _len, set, k) do
    set
  end

  def aux([x | xs], q, k, set, k) do
    aux(xs, :queue.in(x, :queue.drop(q)), k, MapSet.put(set, :queue.to_list(q)), k)
  end

  def aux([x | xs], q, len, set, k) do
    aux(xs, :queue.in(x, q), len + 1, set, k)
  end
end
