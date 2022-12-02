defmodule Solution do
  @spec close_strings(word1 :: String.t(), word2 :: String.t()) :: boolean
  def close_strings(word1, word2) do
    aux(word1) == aux(word2)
  end

  def aux(s) do
    s
    |> String.graphemes()
    |> Enum.reduce(%{}, fn c, acc -> Map.update(acc, c, 1, &(&1 + 1)) end)
    |> then(fn m ->
      {Map.keys(m), Map.values(m) |> Enum.sort()}
    end)
  end
end
