defmodule Solution do
  @spec vowel_strings(words :: [String.t()], left :: integer, right :: integer) :: integer
  def vowel_strings(words, left, right) do
    words
    |> Enum.drop(left)
    |> Enum.take(right - left + 1)
    |> Enum.count(fn w ->
      Regex.match?(~r/^[aeiou](.*[aeiou])?$/, w)
    end)
  end
end
