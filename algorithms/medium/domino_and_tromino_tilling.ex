defmodule Solution do
  @spec num_tilings(n :: integer) :: integer
  def num_tilings(n) do
    dp = %{1 => 1, 2 => 2, 3 => 5}

    4..n//1
    |> Enum.reduce(dp, fn n, dp ->
      v = rem(dp[n - 1] * 2 + dp[n - 3], 1_000_000_007)
      Map.put(dp, n, v)
    end)
    |> Map.get(n)
  end
end
