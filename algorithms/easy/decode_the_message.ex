defmodule Solution do
  @spec decode_message(key :: String.t(), message :: String.t()) :: String.t()
  def decode_message(key, message) do
    key
    |> String.replace(" ", "")
    |> String.to_charlist()
    |> Enum.reduce({%{}, 0}, fn k, {acc, i} ->
      if Map.has_key?(acc, k) do
        {acc, i}
      else
        {Map.put(acc, k, ?a + i), i + 1}
      end
    end)
    |> then(fn {dt, _} ->
      message
      |> String.to_charlist()
      |> Enum.map(&(dt[&1] || &1))
      |> List.to_string()
    end)
  end
end
