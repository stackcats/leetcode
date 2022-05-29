defmodule Solution do
  @spec max_envelopes(envelopes :: [[integer]]) :: integer
  def max_envelopes(envelopes) do
    envelopes
    |> Enum.sort_by(fn [w, h] -> {w, -h} end)
    |> Enum.reduce(%{}, fn [_, h], dp ->
      Map.put(dp, search(dp, h), h)
    end)
    |> map_size()
  end

  def search(dp, target), do: search(dp, 0, map_size(dp) - 1, target)

  def search(dp, l, r, target) when l > r, do: l

  def search(dp, l, r, target) do
    m = div(l + r, 2)

    cond do
      dp[m] == target -> m
      dp[m] < target -> search(dp, m + 1, r, target)
      true -> search(dp, l, m - 1, target)
    end
  end
end
