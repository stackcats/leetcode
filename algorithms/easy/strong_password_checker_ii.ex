defmodule Solution do
  @spec strong_password_checker_ii(password :: String.t()) :: boolean
  def strong_password_checker_ii(password) do
    [
      fn s -> Regex.match?(~r/[a-z]/, s) end,
      fn s -> Regex.match?(~r/[A-Z]/, s) end,
      fn s -> Regex.match?(~r/[0-9]/, s) end,
      fn s -> Regex.match?(~r/[!@#$%^&*()+-]/, s) end,
      fn s -> String.length(s) >= 8 end,
      fn s -> not Regex.match?(~r/(.)\1+/, s) end
    ]
    |> Enum.all?(& &1.(password))
  end
end
