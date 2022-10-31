defmodule Solution do
  @spec average_value(nums :: [integer]) :: integer
  def average_value(nums) do
    nums
    |> Enum.filter(&(rem(&1, 6) == 0))
    |> then(fn ns ->
      if ns == [], do: 0, else: ns |> Enum.sum() |> div(length(ns))
    end)
  end
end
