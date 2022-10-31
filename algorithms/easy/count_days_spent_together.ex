defmodule Solution do
  @spec count_days_together(
          arrive_alice :: String.t(),
          leave_alice :: String.t(),
          arrive_bob :: String.t(),
          leave_bob :: String.t()
        ) :: integer
  def count_days_together(arrive_alice, leave_alice, arrive_bob, leave_bob) do
    alice_start = date_to_days(arrive_alice)
    alice_end = date_to_days(leave_alice)
    bob_start = date_to_days(arrive_bob)
    bob_end = date_to_days(leave_bob)
    max(0, min(alice_end, bob_end) - max(alice_start, bob_start) + 1)
  end

  def date_to_days(s) do
    days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]

    s
    |> String.split("-")
    |> Enum.map(&String.to_integer/1)
    |> then(fn [m, d] -> d + (Enum.take(days, m - 1) |> Enum.sum()) end)
  end
end
