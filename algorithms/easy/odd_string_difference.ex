defmodule Solution do
  @spec odd_string(words :: [String.t()]) :: String.t()
  def odd_string(words) do
    words
    |> Enum.map(fn w ->
      w
      |> String.to_charlist()
      |> diff([])
    end)
    |> Enum.with_index()
    |> Enum.reduce(%{}, fn {arr, i}, acc ->
      Map.update(acc, arr, [i], &[i | &1])
    end)
    |> Enum.find(fn {k, v} -> length(v) == 1 end)
    |> elem(1)
    |> hd()
    |> then(fn i -> Enum.at(words, i) end)
  end

  def diff([_], arr), do: arr

  def diff([a, b | rest], arr) do
    diff([b | rest], [b - a | arr])
  end
end
