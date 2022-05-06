defmodule Solution do
  @spec find_min_arrow_shots(points :: [[integer]]) :: integer
  def find_min_arrow_shots(points) do
    points
    |> Enum.sort(fn [_, e1], [_, e2] -> e1 <= e2 end)
    |> Enum.reduce({-2_147_483_649, 0}, fn [s, e], {acc, ct} ->
      (s <= acc && {acc, ct}) || {e, ct + 1}
    end)
    |> elem(1)
  end
end
