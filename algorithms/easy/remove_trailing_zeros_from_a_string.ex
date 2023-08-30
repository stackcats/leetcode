defmodule Solution do
  @spec remove_trailing_zeros(num :: String.t()) :: String.t()
  def remove_trailing_zeros(num) do
    String.trim_trailing(num, "0")
  end
end
