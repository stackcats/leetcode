defmodule Solution do
  @spec count_prefixes(words :: [String.t()], s :: String.t()) :: integer
  def count_prefixes(words, s) do
    words |> Enum.count(&String.starts_with?(s, &1))
  end
end
