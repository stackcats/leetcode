defmodule Solution do
  @spec check_possibility(nums :: [integer]) :: boolean
  def check_possibility(nums), do: check_possibility(nums, nil, 0)

  def check_possibility(_nums, _a, 2), do: false

  def check_possibility([a, b | ns], nil, ct) do
    check_possibility([b | ns], a, (b < a && ct + 1) || ct)
  end

  def check_possibility([_], _a, ct), do: ct < 2

  def check_possibility([b, c], a, ct) do
    check_possibility([c], b, (c < b && ct + 1) || ct)
  end

  def check_possibility([b, c, d | ns], a, ct) do
    if d < b && a > c do
      false
    else
      check_possibility([c, d | ns], b, (c < b && ct + 1) || ct)
    end
  end
end
