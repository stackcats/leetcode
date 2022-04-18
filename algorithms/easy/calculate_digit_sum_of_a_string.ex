defmodule Solution do
  @spec digit_sum(s :: String.t(), k :: integer) :: String.t()
  def digit_sum(s, k) do
    if String.length(s) <= k do
      s
    else
      s
      |> String.graphemes()
      |> Enum.chunk_every(k)
      |> Enum.map(fn chunk ->
        chunk
        |> Enum.reduce(0, &(String.to_integer(&1) + &2))
        |> to_string()
      end)
      |> Enum.join()
      |> digit_sum(k)
    end
  end
end
