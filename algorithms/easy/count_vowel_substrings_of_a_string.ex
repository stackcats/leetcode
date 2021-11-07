defmodule Solution do
  @spec count_vowel_substrings(word :: String.t()) :: integer
  def count_vowel_substrings(word) do
    vowel_index = %{"a" => -1, "e" => -1, "i" => -1, "o" => -1, "u" => -1}
    word |> String.graphemes() |> aux(0, -1, vowel_index, 0)
  end

  def aux([], _idx, _last_consonant, _vowel_index, ans), do: ans

  def aux([c | rest], idx, last_consonant, vowel_index, ans) do
    if Map.has_key?(vowel_index, c) do
      vowel_index = Map.put(vowel_index, c, idx)
      the_most_left_vowel = vowel_index |> Map.values() |> Enum.min()
      ans = ans + max(0, the_most_left_vowel - last_consonant)
      aux(rest, idx + 1, last_consonant, vowel_index, ans)
    else
      aux(rest, idx + 1, idx, vowel_index, ans)
    end
  end
end
