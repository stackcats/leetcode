defmodule Solution do
  @spec two_city_sched_cost(costs :: [[integer]]) :: integer
  def two_city_sched_cost(costs) do
    n = Enum.count(costs) |> div(2)

    costs
    |> Enum.sort(fn [a0, b0], [a1, b1] -> a0 - b0 < a1 - b1 end)
    |> Enum.with_index()
    |> Enum.reduce(0, fn {[a, b], i}, acc ->
      acc + if i < n, do: a, else: b
    end)
  end
end
