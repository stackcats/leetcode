defmodule Solution do
  @spec minimized_string_length(s :: String.t()) :: integer
  def minimized_string_length(s) do
    s |> String.graphemes() |> MapSet.new() |> MapSet.size()
  end
end
