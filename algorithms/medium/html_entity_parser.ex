defmodule Solution do
  @spec entity_parser(text :: String.t) :: String.t
  def entity_parser(text) do
    decode_map = %{
      "&quot;" => "\"",
      "&apos;" => "'",
      "&amp;" => "&",
      "&gt;" => ">",
      "&lt;" => "<",
      "&frasl;" => "/"
    }
    text |> String.replace(Map.keys(decode_map), & decode_map[&1])
  end
end
