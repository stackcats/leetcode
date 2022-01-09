defmodule Solution do
  @spec capitalize_title(title :: String.t()) :: String.t()
  def capitalize_title(title) do
    title
    |> String.split(" ")
    |> Enum.map(fn s ->
      if String.length(s) < 3, do: String.downcase(s), else: String.capitalize(s)
    end)
    |> Enum.join(" ")
  end
end
