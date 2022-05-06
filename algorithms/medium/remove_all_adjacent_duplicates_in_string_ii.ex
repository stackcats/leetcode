defmodule Solution do
  @spec remove_duplicates(s :: String.t(), k :: integer) :: String.t()
  def remove_duplicates(s, k) do
    s
    |> String.graphemes()
    |> aux([], k)
    |> Enum.map(fn {c, _} -> c end)
    |> Enum.reverse()
    |> Enum.join()
  end

  def aux([], st, _k), do: st

  def aux([x | xs], [], k) do
    aux(xs, [{x, 1}], k)
  end

  def aux([x | xs], st, k) do
    {y, n} = hd(st)

    if x == y do
      if n + 1 == k do
        aux(xs, Enum.drop(st, n), k)
      else
        aux(xs, [{x, n + 1} | st], k)
      end
    else
      aux(xs, [{x, 1} | st], k)
    end
  end
end
