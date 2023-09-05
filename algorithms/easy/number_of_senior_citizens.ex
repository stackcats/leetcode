defmodule Solution do
  @spec count_seniors(details :: [String.t()]) :: integer
  def count_seniors(details) do
    details
    |> Enum.reduce(0, fn d, acc ->
      age = d |> String.slice(11..12) |> String.to_integer()

      if age > 60 do
        acc + 1
      else
        acc
      end
    end)
  end
end
