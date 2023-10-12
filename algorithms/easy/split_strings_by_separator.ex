defmodule Solution do
  @spec split_words_by_separator(words :: [String.t()], separator :: char) :: [String.t()]
  def split_words_by_separator(words, separator) do
    separator = List.to_string([separator])

    words
    |> Enum.flat_map(fn word ->
      word |> String.split(separator) |> Enum.filter(fn w -> w != "" end)
    end)
  end
end
