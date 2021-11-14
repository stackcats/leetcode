defmodule Solution do
  @spec time_required_to_buy(tickets :: [integer], k :: integer) :: integer
  def time_required_to_buy(tickets, k) do
    t = tickets |> Enum.at(k)

    tickets
    |> Enum.with_index()
    |> Enum.reduce(0, fn {n, i}, acc ->
      acc + if i <= k, do: min(n, t), else: min(n, t - 1)
    end)
  end
end
