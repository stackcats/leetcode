defmodule Solution do
  @spec count_elements(nums :: [integer]) :: integer
  def count_elements(nums) do
    {smallest, num_of_smallest, biggest, num_of_biggest} = aux(nums)
    len = Enum.count(nums)

    if smallest == biggest do
      len - num_of_biggest
    else
      len - num_of_biggest - num_of_smallest
    end
  end

  def aux(nums) do
    aux(nums, 100_000, 0, -100_000, 0)
  end

  def aux([], smallest, num_of_smallest, biggest, num_of_biggest) do
    {smallest, num_of_smallest, biggest, num_of_biggest}
  end

  def aux([n | rest], smallest, num_of_smallest, biggest, num_of_biggest) do
    {smallest, num_of_smallest} =
      cond do
        smallest > n -> {n, 1}
        smallest == n -> {n, num_of_smallest + 1}
        true -> {smallest, num_of_smallest}
      end

    {biggest, num_of_biggest} =
      cond do
        biggest < n -> {n, 1}
        biggest == n -> {n, num_of_biggest + 1}
        true -> {biggest, num_of_biggest}
      end

    aux(rest, smallest, num_of_smallest, biggest, num_of_biggest)
  end
end
