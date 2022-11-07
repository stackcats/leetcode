defmodule Solution do
  @spec apply_operations(nums :: [integer]) :: [integer]
  def apply_operations(nums) do
    apply_operations(nums, [], [])
  end

  def apply_operations([], xs, ys) do
    Enum.reverse(xs) ++ ys
  end

  def apply_operations([x], xs, ys) do
    apply_operations([], [x | xs], ys)
  end

  def apply_operations([a, b | rs], xs, ys) do
    cond do
      a == 0 -> apply_operations([b | rs], xs, [0 | ys])
      a == b -> apply_operations(rs, [2 * a | xs], [0 | ys])
      true -> apply_operations([b | rs], [a | xs], ys)
    end
  end
end
