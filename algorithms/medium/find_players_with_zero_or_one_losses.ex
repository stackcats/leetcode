defmodule Solution do
  @spec find_winners(matches :: [[integer]]) :: [[integer]]
  def find_winners(matches) do
    {w, l} =
      matches
      |> Enum.reduce({%{}, %{}}, fn [a, b], {w, l} ->
        {Map.update(w, a, 1, &(&1 + 1)), Map.update(l, b, 1, &(&1 + 1))}
      end)

    [
      Map.keys(w) |> Enum.filter(&(Map.get(l, &1, 0) == 0)) |> Enum.sort(),
      Map.keys(l) |> Enum.filter(&(l[&1] == 1)) |> Enum.sort()
    ]
  end
end
