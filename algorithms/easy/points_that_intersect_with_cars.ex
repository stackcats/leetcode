defmodule Solution do
  @spec number_of_points(nums :: [[integer]]) :: integer
  def number_of_points(nums) do
    mp =
      nums
      |> Enum.reduce(%{}, fn [start, end_], acc ->
        acc
        |> Map.update(start, 1, &(&1 + 1))
        |> Map.update(end_ + 1, -1, &(&1 - 1))
      end)

    1..101
    |> Enum.reduce({0, 0}, fn k, {ans, ct} ->
      ct = ct + Map.get(mp, k, 0)

      if ct > 0 do
        {ans + 1, ct}
      else
        {ans, ct}
      end
    end)
    |> elem(0)
  end
end
