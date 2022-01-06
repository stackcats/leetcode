defmodule Solution do
  @spec car_pooling(trips :: [[integer]], capacity :: integer) :: boolean
  def car_pooling(trips, capacity) do
    m =
      trips
      |> Enum.reduce(%{}, fn [n, from, to], acc ->
        acc
        |> Map.update(from, n, &(&1 + n))
        |> Map.update(to, -n, &(&1 - n))
      end)

    0..1000
    |> Enum.reduce_while({capacity, true}, fn i, {capacity, res} ->
      n = Map.get(m, i, 0)

      if capacity < n do
        {:halt, {capacity, false}}
      else
        {:cont, {capacity - n, true}}
      end
    end)
    |> elem(1)
  end
end
