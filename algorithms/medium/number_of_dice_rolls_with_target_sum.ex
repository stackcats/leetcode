defmodule Solution do
  @mod 1_000_000_007

  @spec num_rolls_to_target(n :: integer, k :: integer, target :: integer) :: integer
  def num_rolls_to_target(n, k, target) do
    dfs(n, target, k)
    |> elem(0)
  end

  def dfs(left, target, k, dp \\ %{}) do
    cond do
      left == 0 && target == 0 ->
        {1, dp}

      left == 0 ->
        {0, dp}

      target < 0 ->
        {0, dp}

      dp[{left, target}] != nil ->
        {dp[{left, target}], dp}

      true ->
        1..k
        |> Enum.reduce({0, dp}, fn f, {acc, dp} ->
          {ct, dp} = dfs(left - 1, target - f, k, dp)
          {rem(acc + ct, @mod), dp}
        end)
        |> then(fn {ct, dp} ->
          {ct, Map.put(dp, {left, target}, ct)}
        end)
    end
  end
end
