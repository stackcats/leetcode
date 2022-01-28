defmodule Solution do
  @spec number_of_weak_characters(properties :: [[integer]]) :: integer
  def number_of_weak_characters(properties) do
    properties
    |> Enum.sort(fn [a0, a1], [b0, b1] ->
      if a0 == b0, do: b1 <= a1, else: a0 <= b0
    end)
    |> Enum.reverse()
    |> Enum.reduce({0, 0}, fn [_, d], {max, ans} ->
      if d < max, do: {max, ans + 1}, else: {d, ans}
    end)
    |> elem(1)
  end
end
