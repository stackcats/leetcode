defmodule Solution do
  @spec maximum_value(strs :: [String.t()]) :: integer
  def maximum_value(strs) do
    strs
    |> Enum.map(fn s ->
      try do
        String.to_integer(s)
      rescue
        ArgumentError -> String.length(s)
      end
    end)
    |> Enum.max()
  end
end
