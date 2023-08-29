defmodule Solution do
  @spec find_the_prefix_common_array(a :: [integer], b :: [integer]) :: [integer]
  def find_the_prefix_common_array(a, b) do
    Enum.zip(a, b)
    |> Enum.reduce({[], 0, %MapSet{}, %MapSet{}}, fn {a, b}, {ans, ct, sa, sb} ->
      ct =
        cond do
          a == b -> ct + 1
          MapSet.member?(sa, b) && MapSet.member?(sb, a) -> ct + 2
          MapSet.member?(sa, b) || MapSet.member?(sb, a) -> ct + 1
          true -> ct
        end

      {[ct | ans], ct, MapSet.put(sa, a), MapSet.put(sb, b)}
    end)
    |> elem(0)
    |> Enum.reverse()
  end
end
