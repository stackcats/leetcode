defmodule Solution do
  @spec maximum_difference(nums :: [integer]) :: integer
  def maximum_difference([x|xs]) do
    helper(xs, x, -1)
  end

  def helper([], _, ans), do: ans
  def helper([x|xs], y, ans) do
    if x <= y do
      helper(xs, x, ans)
    else
      helper(xs, y, max(ans, x - y))
    end
  end
end
