defmodule Solution do
  @spec maximum_number_of_string_pairs(words :: [String.t()]) :: integer
  def maximum_number_of_string_pairs(words) do
    mp =
      words
      |> Enum.reduce(%{}, fn word, acc -> Map.update(acc, word, 1, &(&1 + 1)) end)

    words
    |> Enum.reduce({0, mp}, fn word, {ct, mp} ->
      rev = String.reverse(word)
      mp = Map.update!(mp, word, &(&1 - 1))

      {ct + if(Map.get(mp, rev, 0) > 0, do: 1, else: 0), mp}
    end)
    |> elem(0)
  end
end
