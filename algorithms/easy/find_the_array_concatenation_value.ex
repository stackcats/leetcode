defmodule Solution do
  @spec find_the_array_conc_val(nums :: [integer]) :: integer
  def find_the_array_conc_val(nums) do
    len = length(nums) |> div(2)
    {f, r} = nums |> Enum.split(len)
    aux(f, Enum.reverse(r), 0)
  end

  def aux([], [], ans), do: ans
  def aux([], [y], ans), do: ans + y

  def aux([x | xs], [y | ys], ans) do
    aux(xs, ys, ans + String.to_integer("#{x}#{y}"))
  end
end
