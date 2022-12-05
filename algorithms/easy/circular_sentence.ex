defmodule Solution do
  @spec is_circular_sentence(sentence :: String.t()) :: boolean
  def is_circular_sentence(sentence) do
    lst =
      sentence
      |> String.split(" ")
      |> Enum.map(fn s ->
        lst = s |> String.graphemes()
        [List.first(lst), List.last(lst)]
      end)

    (lst ++ [hd(lst)])
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.all?(fn [[_, t], [f, _]] -> t == f end)
  end
end
