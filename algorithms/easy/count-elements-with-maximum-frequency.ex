defmodule Solution do
  @spec max_frequency_elements(nums :: [integer]) :: integer
  def max_frequency_elements(nums) do
    nums
    |> Enum.frequencies()
    |> Enum.reduce({0, 0}, fn {k, v}, {acc, ma} ->
      cond do
        v == ma -> {acc + v, ma}
        v < ma -> {acc, ma}
        v > ma -> {v, v}
      end
    end)
    |> elem(0)
  end
end
