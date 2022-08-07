defmodule Solution do
  @m 1_000_000_007

  @spec count_vowel_permutation(n :: integer) :: integer
  def count_vowel_permutation(n) do
    count_vowel_permutation({1, 1, 1, 1, 1}, n)
  end

  def count_vowel_permutation({a, e, i, o, u}, 1) do
    rem(a + e + i + o + u, @m)
  end

  def count_vowel_permutation({a, e, i, o, u}, n) do
    na = rem(e + i + u, @m)
    ne = rem(a + i, @m)
    ni = rem(e + o, @m)
    no = i
    nu = rem(i + o, @m)
    count_vowel_permutation({na, ne, ni, no, nu}, n - 1)
  end
end
