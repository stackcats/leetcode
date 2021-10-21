defmodule Solution do
  @spec are_numbers_ascending(s :: String.t()) :: boolean
  def are_numbers_ascending(s) do
    lst =
      s
      |> String.split(" ")
      |> Enum.map(&Integer.parse/1)
      |> Enum.filter(&(&1 != :error))
      |> is_ascending()
  end

  defp is_ascending([]), do: false
  defp is_ascending([x]), do: true

  defp is_ascending([x, y | rest]) do
    if x < y do
      is_ascending([y | rest])
    else
      false
    end
  end
end
