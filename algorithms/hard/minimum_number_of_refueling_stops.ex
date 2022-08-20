defmodule Solution do
  @spec min_refuel_stops(target :: integer, start_fuel :: integer, stations :: [[integer]]) ::
          integer
  def min_refuel_stops(target, start_fuel, stations) do
    (stations ++ [[target, 0]])
    |> Enum.reduce_while({0, start_fuel, :gb_sets.new()}, fn station, {ans, tank, pq} ->
      refuel(tank, station, pq, ans)
    end)
    |> elem(0)
  end

  def refuel(tank, [pos, fuel], pq, ans) when tank >= pos do
    # can't simply add(fuel, pq) because gb_sets ignore duplicate fuel
    {:cont, {ans, tank, :gb_sets.add({fuel, pos}, pq)}}
  end

  def refuel(tank, [pos, fuel], pq, ans) do
    if :gb_sets.is_empty(pq) do
      {:halt, {-1}}
    else
      {{f, _}, pq} = :gb_sets.take_largest(pq)
      refuel(tank + f, [pos, fuel], pq, ans + 1)
    end
  end
end
