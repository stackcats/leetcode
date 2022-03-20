defmodule Solution do
  @spec divide_array(nums :: [integer]) :: boolean
  def divide_array(nums) do
    nums
    |> Enum.reduce(%{}, fn n, acc ->
      Map.update(acc, n, 1, &(&1 + 1))
    end)
    |> Enum.all?(fn {_, ct} -> rem(ct, 2) == 0 end)
  end
end
