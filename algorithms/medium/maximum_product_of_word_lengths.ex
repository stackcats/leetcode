defmodule Solution do
  use Bitwise

  @spec max_product(words :: [String.t]) :: integer
  def gen_bits(s) do
    s |> to_char_list |> Enum.reduce(0, fn x, acc -> acc ||| (1 <<< (x - 97)) end)
  end
  
  def max_product(words) do
    bits = words |> Enum.with_index |> Map.new(fn {n, i} -> {i, gen_bits(n)} end)
    m_words = words |> Enum.with_index |> Map.new(fn {n, i} -> {i, String.length(n)} end)
    helper(0, 1, Enum.count(words), m_words, bits, 0)
  end
  
  def helper(i, j, len, words, bits, ans) do
    cond do
      i == len -> ans
      j == len -> helper(i + 1, 0, len, words, bits, ans)
      true ->
        if (bits[i] &&& bits[j]) == 0 do
          helper(i, j + 1, len, words, bits, max(ans, words[i] * words[j]))
        else
          helper(i, j + 1, len, words, bits, ans)
        end
    end
  end
end
