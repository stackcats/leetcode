defmodule Solution do
  @spec valid_ip_address(ip :: String.t) :: String.t
  def valid_ip_address(ip) do
    cond do
      Regex.match?(~r/^((25[0-5]|2[0-4][0-9]|1[0-9]{2}|1[0-9]|[0-9]).){3}(25[0-5]|2[0-4][0-9]|1[0-9]{2}|1[0-9]|[0-9])$/, ip) -> "IPv4"
      Regex.match?(~r/^([0-9a-f]{1,4}:){7}[0-9a-f]{1,4}$/i, ip) -> "IPv6"
      true -> "Neither"
    end
  end
end
