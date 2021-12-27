defmodule Solution do
  @spec most_words_found(sentences :: [String.t()]) :: integer
  def most_words_found(sentences) do
    sentences
    |> Enum.map(fn s -> String.split(s, " ") |> Enum.count() end)
    |> Enum.max()
  end
end
