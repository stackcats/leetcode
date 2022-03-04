defmodule Solution do
  @spec prefix_count(words :: [String.t()], pref :: String.t()) :: integer
  def prefix_count(words, pref) do
    words |> Enum.filter(&String.starts_with?(&1, pref)) |> Enum.count()
  end
end
