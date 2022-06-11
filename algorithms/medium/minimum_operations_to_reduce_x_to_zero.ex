defmodule Solution do
  @spec min_operations(nums :: [integer], x :: integer) :: integer
  def min_operations(nums, x) do
    y = Enum.sum(nums) - x
    len = Enum.count(nums)

    if y == 0 do
      len
    else
      nums
      |> Enum.with_index()
      |> Enum.reduce({-1, %{0 => -1}, 0}, fn {n, i}, {ans, m, acc} ->
        acc = acc + n

        {
          max(ans, i - Map.get(m, acc - y, i + 1)),
          Map.put(m, acc, i),
          acc
        }
      end)
      |> then(fn {ans, _, _} ->
        (ans == -1 && -1) || len - ans
      end)
    end
  end
end
