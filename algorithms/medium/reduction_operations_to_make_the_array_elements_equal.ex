defmodule Solution do
  @spec reduction_operations(nums :: [integer]) :: integer
  def reduction_operations(nums) do
    nums
    |> Enum.sort(&>=/2)
    |> Enum.reduce({0, 0, 0}, fn n, {acc, prev, ct} ->
      if n == prev do
        {acc, prev, ct + 1}
      else
        {acc + ct, n, ct + 1}
      end
    end)
    |> elem(0)
  end
end
