defmodule Solution do
  @spec final_string(s :: String.t()) :: String.t()
  def final_string(s) do
    s
    |> String.graphemes()
    |> Enum.reduce("", fn c, acc ->
      if c == "i" do
        String.reverse(acc)
      else
        acc <> c
      end
    end)
  end
end
