defmodule Solution do
  @spec get_smallest_string(n :: integer, k :: integer) :: String.t()
  def get_smallest_string(n, k) do
    aux(n, k)
  end

  def aux(n, k), do: aux(n, k, [])
  def aux(0, _k, s), do: List.to_string(s)

  def aux(n, k, s) do
    c = min(26, k - n + 1)
    aux(n - 1, k - c, [97 + c - 1 | s])
  end
end
