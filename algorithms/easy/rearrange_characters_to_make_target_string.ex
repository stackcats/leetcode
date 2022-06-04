defmodule Solution do
  @spec rearrange_characters(s :: String.t(), target :: String.t()) :: integer
  def rearrange_characters(s, target) do
    ct = counter(s)

    counter(target)
    |> Enum.reduce_while(101, fn {k, v}, acc ->
      if ct[k] do
        {:cont, min(acc, div(ct[k], v))}
      else
        {:halt, 0}
      end
    end)
  end

  def counter(s) do
    s
    |> String.graphemes()
    |> Enum.reduce(%{}, fn c, m ->
      Map.update(m, c, 1, &(&1 + 1))
    end)
  end
end
