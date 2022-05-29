defmodule Solution do
  @spec digit_count(num :: String.t()) :: boolean
  def digit_count(num) do
    num
    |> String.graphemes()
    |> Enum.map(&String.to_integer/1)
    |> Enum.with_index()
    |> Enum.reduce({%{}, %{}}, fn {d, i}, {m, n} ->
      {Map.update(m, d, 1, &(&1 + 1)), Map.put(n, i, d)}
    end)
    |> then(fn {m, n} ->
      Enum.all?(m, fn {k, v} -> n[k] == v end)
    end)
  end
end
