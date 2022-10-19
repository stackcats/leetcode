defmodule Solution do
  @spec equal_frequency(word :: String.t()) :: boolean
  def equal_frequency(word) do
    word
    |> String.graphemes()
    |> Enum.reduce(%{}, fn c, acc ->
      Map.update(acc, c, 1, &(&1 + 1))
    end)
    |> then(fn map ->
      map
      |> Enum.any?(fn {k, v} ->
        map
        |> Map.update!(k, &(&1 - 1))
        |> Map.values()
        |> Enum.filter(&(&1 != 0))
        |> Enum.uniq()
        |> length() == 1
      end)
    end)
  end
end
