defmodule Solution do
  @spec construct2_d_array(original :: [integer], m :: integer, n :: integer) :: [[integer]]
  def construct2_d_array(original, m, n) do
    if Enum.count(original) != m * n do
      []
    else
      Enum.chunk_every(original, n)
    end
  end
end
