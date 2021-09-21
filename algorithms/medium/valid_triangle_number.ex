defmodule Solution do
  @spec triangle_number(nums :: [integer]) :: integer
  def triangle_number(nums) do
    len = Enum.count(nums)
    if len < 3 do
      0
    else
      tbl = nums 
      |> Enum.sort 
      |> Enum.with_index
      |> Map.new(fn({x, i}) ->
        {i, x}
      end)
      loop_nums(tbl, 2, len)     
    end
  end
  
  def loop_nums(tbl, i, len) do
    if i == len do
      0
    else
      check_intervals(tbl, 0, i - 1, i) + loop_nums(tbl, i + 1, len)
    end
  end
  
  def check_intervals(tbl, a, b, c) do
    cond do
      a >= b -> 0
      Map.fetch!(tbl, a) + Map.fetch!(tbl, b) > Map.fetch!(tbl, c) -> b - a + check_intervals(tbl, a, b - 1, c)
      true -> check_intervals(tbl, a + 1, b, c)
    end
  end
end
