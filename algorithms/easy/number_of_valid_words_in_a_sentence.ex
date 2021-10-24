defmodule Solution do
  @spec count_valid_words(sentence :: String.t()) :: integer
  def count_valid_words(sentence) do
    sentence
    |> String.split(" ", trim: true)
    |> Enum.filter(&Regex.match?(~r/^([a-z]+-[a-z]+)?[a-z]*(!|\.|,)?$/, &1))
    |> Enum.count()
  end
end
