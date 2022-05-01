defmodule Solution do
  use Bitwise
  @spec single_number(nums :: [integer]) :: [integer]
  def single_number(nums) do
    r = nums |> Enum.reduce(0, &bxor/2) |> then(&band(&1, -&1))

    nums
    |> Enum.reduce({0, 0}, fn n, {a, b} ->
      case band(n, r) do
        0 -> {bxor(a, n), b}
        _ -> {a, bxor(b, n)}
      end
    end)
    |> Tuple.to_list()
  end
end
