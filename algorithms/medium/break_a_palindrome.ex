defmodule Solution do
  @spec break_palindrome(palindrome :: String.t()) :: String.t()
  def break_palindrome(palindrome) do
    len = String.length(palindrome)

    if len == 1 do
      ""
    else
      palindrome
      |> String.to_charlist()
      |> then(fn lst ->
        ndx = Enum.find_index(lst, fn c -> c != ?a end)

        if is_mid(ndx, len) || ndx == nil do
          List.replace_at(lst, len - 1, ?b)
        else
          List.replace_at(lst, ndx, ?a)
        end
      end)
      |> List.to_string()
    end
  end

  def is_mid(i, len) do
    rem(len, 2) == 1 && i == div(len, 2)
  end
end
