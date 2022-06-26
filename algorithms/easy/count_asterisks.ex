defmodule Solution do
  @spec count_asterisks(s :: String.t()) :: integer
  def count_asterisks(s) do
    s
    |> String.graphemes()
    |> Enum.reduce({false, 0}, fn c, {is_exclude, sum} ->
      case c do
        "|" -> {not is_exclude, sum}
        "*" -> {is_exclude, (is_exclude && sum) || sum + 1}
        _ -> {is_exclude, sum}
      end
    end)
    |> elem(1)
  end
end
