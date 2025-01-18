defmodule Solution do
  @spec has_match(s :: String.t(), p :: String.t()) :: boolean
  def has_match(s, p) do
    String.replace(p, "*", ".*")
    |> Regex.compile!()
    |> Regex.match?(s)
  end
end
