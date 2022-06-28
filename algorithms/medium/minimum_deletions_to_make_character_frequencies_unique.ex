defmodule Solution do
  @spec min_deletions(s :: String.t()) :: integer
  def min_deletions(s) do
    s
    |> String.to_charlist()
    |> Enum.reduce(%{}, fn c, acc ->
      Map.update(acc, c, 1, &(&1 + 1))
    end)
    |> Map.values()
    |> Enum.sort(&>=/2)
    |> then(fn vs ->
      Enum.reduce_while(vs, {100_000, 0}, fn v, {pre, ct} ->
        cond do
          v == 0 ->
            {:halt, {pre, ct}}

          v >= pre ->
            pre = max(0, pre - 1)
            {:cont, {pre, ct + v - pre}}

          true ->
            {:cont, {v, ct}}
        end
      end)
      |> elem(1)
    end)
  end
end
