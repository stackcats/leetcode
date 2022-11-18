defmodule Solution do
  @spec smallest_number(pattern :: String.t()) :: String.t()
  def smallest_number(pattern) do
    pattern
    |> String.graphemes()
    |> aux(?1, [], [])
    |> List.to_string()
  end

  def aux([], n, st, ans) do
    ans ++ [n | st]
  end

  def aux([c | cs], n, st, ans) do
    st = [n | st]

    case c do
      "I" -> aux(cs, n + 1, [], ans ++ st)
      "D" -> aux(cs, n + 1, st, ans)
    end
  end
end
