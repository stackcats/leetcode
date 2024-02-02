defmodule Solution do
  import Bitwise
  @spec minimum_one_bit_operations(n :: integer) :: integer
  def minimum_one_bit_operations(n) do
    inverse_gray(n, 0)
  end

  def inverse_gray(0, ct), do: ct

  def inverse_gray(n, ct) do
    inverse_gray(n >>> 1, bxor(ct, n))
  end
end

