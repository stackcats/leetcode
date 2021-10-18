defmodule Solution do
  @spec is_interleave(s1 :: String.t(), s2 :: String.t(), s3 :: String.t()) :: boolean
  def is_interleave(s1, s2, s3) do
    len1 = String.length(s1)
    len2 = String.length(s2)
    len3 = String.length(s3)

    if len1 + len2 != len3 do
      false
    else
      dp =
        for i <- 0..len1, reduce: %{} do
          acc ->
            for j <- 0..len2, reduce: acc do
              acc -> Map.put(acc, {i, j}, false)
            end
        end

      dp = Map.put(dp, {0, 0}, true)

      dp =
        for i <- 1..len1//1, reduce: dp do
          acc ->
            Map.put(acc, {i, 0}, acc[{i - 1, 0}] && String.at(s1, i - 1) == String.at(s3, i - 1))
        end

      dp =
        for j <- 1..len2//1, reduce: dp do
          acc ->
            Map.put(acc, {0, j}, acc[{0, j - 1}] && String.at(s2, j - 1) == String.at(s3, j - 1))
        end

      dp =
        for i <- 1..len1//1, reduce: dp do
          acc ->
            for j <- 1..len2//1, reduce: acc do
              acc ->
                Map.put(
                  acc,
                  {i, j},
                  (acc[{i - 1, j}] && String.at(s1, i - 1) == String.at(s3, i + j - 1)) ||
                    (acc[{i, j - 1}] && String.at(s2, j - 1) == String.at(s3, i + j - 1))
                )
            end
        end

      dp[{len1, len2}]
    end
  end
end
