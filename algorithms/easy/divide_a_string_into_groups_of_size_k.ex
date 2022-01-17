defmodule Solution do
  @spec divide_string(s :: String.t(), k :: integer, fill :: char) :: [String.t()]
  def divide_string(s, k, fill) do
    re = rem(String.length(s), k)

    s = if re != 0, do: s <> String.duplicate(to_string([fill]), k - re), else: s

    String.graphemes(s) |> Enum.chunk_every(k) |> Enum.map(&Enum.join/1)
  end
end
