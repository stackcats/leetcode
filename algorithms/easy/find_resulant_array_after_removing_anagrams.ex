defmodule Solution do
  @spec remove_anagrams(words :: [String.t()]) :: [String.t()]
  def remove_anagrams([w | ws]) do
    remove_anagrams(ws, [w], to_map(w))
  end

  def remove_anagrams([], lst, pre) do
    Enum.reverse(lst)
  end

  def remove_anagrams([w | ws], [y | ys], pre) do
    sw = to_map(w)

    if sw == pre do
      remove_anagrams(ws, [y | ys], pre)
    else
      remove_anagrams(ws, [w, y | ys], sw)
    end
  end

  def to_map(w) do
    w
    |> String.graphemes()
    |> Enum.reduce(%{}, fn c, acc ->
      Map.update(acc, c, 1, &(&1 + 1))
    end)
  end
end
