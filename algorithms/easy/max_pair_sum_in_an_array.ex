defmodule Solution do
  @spec max_sum(nums :: [integer]) :: integer
  def max_sum(nums) do
    nums
    |> Enum.reduce(%{}, fn n, acc ->
      k = max_digit(n)

      Map.update(acc, k, [n], fn vs ->
        [n | vs]
        |> Enum.sort(:desc)
        |> Enum.take(2)
      end)
    end)
    |> Map.values()
    |> Enum.flat_map(fn lst ->
      case lst do
        [a, b] -> [a + b]
        _ -> []
      end
    end)
    |> Enum.max(&>=/2, fn -> -1 end)
  end

  @spec max_digit(num :: integer) :: integer
  defp max_digit(num, ma \\ 0)
  defp max_digit(0, ma), do: ma

  defp max_digit(num, ma) do
    max_digit(div(num, 10), max(rem(num, 10), ma))
  end
end
