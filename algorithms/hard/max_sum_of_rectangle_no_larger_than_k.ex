defmodule Solution do
  @spec max_sum_submatrix(matrix :: [[integer]], k :: integer) :: integer
  def max_sum_submatrix(matrix, k) do
    m = length(matrix) - 1
    n = length(hd(matrix)) - 1

    matrix =
      matrix
      |> Enum.with_index()
      |> Enum.reduce(%{}, fn {row, i}, acc ->
        row
        |> Enum.with_index()
        |> Enum.reduce(acc, fn {n, j}, acc ->
          if j == 0 do
            Map.put(acc, {i, j}, n)
          else
            Map.put(acc, {i, j}, n + acc[{i, j - 1}])
          end
        end)
      end)

    for left <- 0..n, right <- left..n, reduce: -100_000 do
      ans ->
        for r <- 0..m, reduce: {ans, 0, :gb_sets.from_list([0])} do
          {ans, pre_sum, set} ->
            pre_sum = pre_sum + matrix[{r, right}] - Map.get(matrix, {r, left - 1}, 0)
            iter = :gb_sets.iterator_from(pre_sum - k, set)

            ans =
              case :gb_sets.next(iter) do
                {sum, _} -> max(ans, pre_sum - sum)
                _ -> ans
              end

            set = :gb_sets.add(pre_sum, set)
            {ans, pre_sum, set}
        end
        |> elem(0)
    end
  end
end
