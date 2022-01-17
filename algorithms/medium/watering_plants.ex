defmodule Solution do
  @spec watering_plants(plants :: [integer], capacity :: integer) :: integer
  def watering_plants(plants, capacity) do
    plants
    |> Enum.with_index()
    |> Enum.reduce({capacity, 0}, fn {n, i}, {cap, acc} ->
      if cap >= n do
        {cap - n, acc + 1}
      else
        {capacity - n, acc + i * 2 + 1}
      end
    end)
    |> elem(1)
  end
end
