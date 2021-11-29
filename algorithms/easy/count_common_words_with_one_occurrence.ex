defmodule Solution do
  @spec count_words(words1 :: [String.t()], words2 :: [String.t()]) :: integer
  def count_words(words1, words2) do
    dt1 =
      words1
      |> Enum.reduce(%{}, fn w, dt ->
        Map.update(dt, w, 1, &(&1 + 1))
      end)

    dt2 =
      words2
      |> Enum.reduce(%{}, fn w, dt ->
        Map.update(dt, w, 1, &(&1 + 1))
      end)

    dt1
    |> Enum.reduce(0, fn {k, v}, acc ->
      acc + if v == 1 && Map.get(dt2, k, 0) == 1, do: 1, else: 0
    end)
  end
end
