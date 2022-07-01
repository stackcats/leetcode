defmodule Solution do
  @spec maximum_units(box_types :: [[integer]], truck_size :: integer) :: integer
  def maximum_units(box_types, truck_size) do
    box_types
    |> Enum.sort_by(fn [_, k] -> -k end)
    |> Enum.reduce_while({0, 0}, fn [n, x], {total, ans} ->
      cond do
        total >= truck_size -> {:halt, {total, ans}}
        total + n <= truck_size -> {:cont, {total + n, ans + n * x}}
        true -> {:cont, {truck_size, ans + (truck_size - total) * x}}
      end
    end)
    |> elem(1)
  end
end
