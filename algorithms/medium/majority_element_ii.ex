defmodule Solution do
  @spec majority_element(nums :: [integer]) :: [integer]
  def majority_element(nums) do
    x = nums |> length() |> div(3)

    nums
    |> Enum.reduce([{0, 0}, {1, 0}], fn n, [{c1, ct1}, {c2, ct2}] ->
      cond do
        c1 == n -> [{c1, ct1 + 1}, {c2, ct2}]
        c2 == n -> [{c1, ct1}, {c2, ct2 + 1}]
        ct1 == 0 -> [{n, 1}, {c2, ct2}]
        ct2 == 0 -> [{c1, ct1}, {n, 1}]
        true -> [{c1, ct1 - 1}, {c2, ct2 - 1}]
      end
    end)
    |> Enum.flat_map(fn {c, _} ->
      (Enum.count(nums, &(&1 == c)) > x && [c]) || []
    end)
  end
end
