defmodule RustlerLiveViewToy.Engine.TCP do
  @delimiter ";"

  def connect(:localhost, port_number), do: connect('localhost', port_number)

  def connect(host, port_number) do
    :gen_tcp.connect(host, port_number, [:binary])
  end

  def send(port, message) do
    :gen_tcp.send(port, "#{message}#{@delimiter}")
  end
end
