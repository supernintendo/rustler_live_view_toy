defmodule RustlerLiveViewToyWeb.GameLive do
  use Phoenix.LiveView
  alias RustlerLiveViewToyWeb.{AudioLive, GraphicsLive, InputLive}
  require Logger

  @default_assigns %{
    avatars: %{},
    controls: %{
      "w" => {"MoveUp", "HaltMoveUp"},
      "a" => {"MoveLeft", "HaltMoveLeft"},
      "s" => {"MoveDown", "HaltMoveDown"},
      "d" => {"MoveRight", "HaltMoveRight"},
      "leftMouse" => {"Shoot", "HaltShoot"}
    }
  }

  def render(assigns) do
    ~L"""
    <%= AudioLive.render(assigns) %>
    <%= GraphicsLive.render(assigns) %>
    <%= InputLive.render(assigns) %>
    """
  end

  def mount(_session, socket) do
    with true <- connected?(socket),
         {:ok, engine} <- :gen_tcp.connect('localhost', 6142, [:binary]) do
      {:ok, default_state(socket, engine)}
    else
      _ ->
        {:ok, socket}
    end
  end

  def default_state(socket, engine) do
    socket
    |> assign(@default_assigns)
    |> assign(%{
      avatars: get_avatars(),
      id: Ecto.UUID.generate(),
      engine: engine
    })
    |> start_engine()
  end

  def get_avatars() do
    :ets.tab2list(:rustler_live_view_toy_avatars) |> Enum.into(%{})
  end

  def handle_event("keydown", %{"key" => key}, %{assigns: %{controls: %{} = kb}} = socket) do
    socket
    |> handle_key(kb[key], :keydown)
    |> noreply()
  end

  def handle_event("keyup", %{"key" => key}, %{assigns: %{controls: %{} = kb}} = socket) do
    socket
    |> handle_key(kb[key], :keyup)
    |> noreply()
  end

  def handle_event(_event_name, _event, socket) do
    {:noreply, socket}
  end

  def handle_key(%{} = socket, {keydown_message, _}, :keydown) do
    relay(socket, "#{keydown_message}")
  end

  def handle_key(%{} = socket, {_, keyup_message}, :keyup) do
    relay(socket, "#{keyup_message}")
  end

  def handle_key(socket, _message_tuple, _key_event), do: socket

  def handle_info({:cache, table, {ets_table, id}}, socket) do
    case :ets.lookup(ets_table, id) do
      [{_id, record}] ->
        socket
        |> assign(table, Map.put(socket.assigns[table], id, record))
        |> noreply()

      _ ->
        socket
        |> assign(table, Map.delete(socket.assigns[table], id))
        |> noreply()
    end
  end

  def handle_info(_message, socket) do
    noreply(socket)
  end

  defp noreply(socket), do: {:noreply, socket}

  defp relay(%{assigns: %{id: _id, engine: engine}} = socket, message) do
    :gen_tcp.send(engine, "#{message};")

    socket
  end

  defp start_engine(%{assigns: %{id: id, engine: engine}} = socket) do
    :ets.insert(:rustler_live_view_toy_players, {id, self()})
    :gen_tcp.send(engine, "#{id};")

    socket
  end
end
