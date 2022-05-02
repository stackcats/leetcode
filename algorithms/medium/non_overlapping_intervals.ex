defmodule Solution do
  @spec erase_overlap_intervals(intervals :: [[integer]]) :: integer
  def erase_overlap_intervals(intervals) do
    intervals
    |> Enum.sort(fn [_, e1], [_, e2] -> e1 <= e2 end)
    |> Enum.reduce({-50000, 0}, fn [s, e], {acc, ct} ->
      (s < acc && {acc, ct + 1}) || {e, ct}
    end)
    |> elem(1)
  end
end
