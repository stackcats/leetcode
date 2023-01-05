defmodule Solution do
  @spec integer_break(n :: integer) :: integer
  def integer_break(n) do
    2..n
    |> Enum.reduce(%{1 => 1}, fn i, dp ->
      1..div(i, 2)
      |> Enum.reduce(dp, fn j, dp ->
        [
          dp[j] * dp[i - j],
          j * dp[i - j],
          dp[j] * (i - j),
          j * (i - j)
        ]
        |> Enum.max()
        |> then(fn m ->
          Map.update(dp, i, m, &max(&1, m))
        end)
      end)
    end)
    |> Map.values()
    |> Enum.max()
  end
end
