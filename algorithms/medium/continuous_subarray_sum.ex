defmodule Solution do
  @spec check_subarray_sum(nums :: [integer], k :: integer) :: boolean
  def check_subarray_sum(nums, k) do
    nums |> Enum.with_index() |> helper(k, 0, %{0 => -1})
  end

  def helper([], _, _, _), do: false

  def helper([{n, i} | rest], k, sum, m) do
    sum = sum + n
    r = rem(sum, k)

    case Map.fetch(m, r) do
      {:ok, j} -> if i - m[r] > 1, do: true, else: helper(rest, k, sum, m)
      _ -> helper(rest, k, sum, Map.put(m, r, i))
    end
  end
end
