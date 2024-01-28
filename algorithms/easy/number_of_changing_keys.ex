defmodule Solution do
  @spec count_key_changes(s :: String.t()) :: integer
  def count_key_changes(s) do
    [x | xs] =
      s
      |> String.downcase()
      |> String.graphemes()

    Enum.reduce(xs, {0, x}, fn x, {acc, prev} ->
      acc = acc + if x == prev, do: 0, else: 1
      {acc, x}
    end)
    |> elem(0)
  end
end
