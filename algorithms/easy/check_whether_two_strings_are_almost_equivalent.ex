defmodule Solution do
  @spec check_almost_equivalent(word1 :: String.t(), word2 :: String.t()) :: boolean
  def check_almost_equivalent(word1, word2) do
    m1 = gen_map(word1)
    m2 = gen_map(word2)
    n = larger_than_three(m1, m2)
    m = larger_than_three(m2, m1)
    n == 0 && m == 0
  end

  defp gen_map(s) do
    s
    |> String.graphemes()
    |> Enum.reduce(%{}, fn c, acc ->
      Map.update(acc, c, 1, &(&1 + 1))
    end)
  end

  defp larger_than_three(m1, m2) do
    m1
    |> Enum.filter(fn {k, v} ->
      abs(v - Map.get(m2, k, 0)) > 3
    end)
    |> Enum.count()
  end
end
