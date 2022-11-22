defmodule Solution do
  @spec num_squares(n :: integer) :: integer
  def num_squares(n) do
    aux(%{0 => 0}, n)
  end

  def aux(dp, n) when map_size(dp) > n, do: dp[n]

  def aux(dp, n) do
    m = map_size(dp)

    for i <- 1..trunc(:math.sqrt(m)), reduce: 10000 do
      acc ->
        min(acc, dp[m - i * i] + 1)
    end
    |> then(fn ct ->
      dp
      |> Map.put(m, ct)
      |> aux(n)
    end)
  end
end
