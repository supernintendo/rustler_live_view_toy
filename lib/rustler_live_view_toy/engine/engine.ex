defmodule RustlerLiveViewToy.Engine do
  use GenServer
  use Rustler, otp_app: :rustler_live_view_toy, crate: "rustler_live_view_toy_engine"
  require Logger
  require Sage

  alias RustlerLiveViewToy.Game.Avatar
  alias RustlerLiveViewToy.Engine.{Config, TCP}

  # When your NIF is loaded, it will override this function.
  def start(%Config{} = _config), do: {:error, :nif_not_loaded}

  def start_link do
    GenServer.start_link(__MODULE__, :ok, [])
  end

  @impl true
  def init(:ok) do
    Sage.new()
    |> Sage.run(:config, &prepare_config/2)
    |> Sage.run(:engine, &start_engine/2)
    |> Sage.run(:ets, &start_ets/2)
    |> Sage.run(:tcp, &connect_tcp/2, &retry_tcp/4)
    |> Sage.run(:master, &verify_master/2)
    |> Sage.execute()
    |> case do
      {:ok, _last_result, state} ->
        Logger.info("#{inspect(__MODULE__)} NIF started.")
        {:ok, state}

      error ->
        raise "#{__MODULE__}: &init/1 failure - #{inspect(error)}"
    end
  end

  def prepare_config(_effects_so_far, _opts) do
    {:ok,
     %Config{
       master_key: Ecto.UUID.generate()
     }}
  end

  def start_engine(%{config: %Config{} = config}, _opts) do
    __MODULE__.start(config)
  end

  def start_ets(_effects_so_far, _opts) do
    {:ok,
     %{
       avatars: :ets.new(:rustler_live_view_toy_avatars, [:named_table, :public]),
       players: :ets.new(:rustler_live_view_toy_players, [:named_table, :public])
     }}
  end

  def connect_tcp(%{config: %Config{tcp_port: port_number}}, _opts) do
    TCP.connect(:localhost, port_number)
  end

  def retry_tcp(_effect_to_compensate, _effects_so_far, _failed_stage, _opts) do
    {:retry, retry_limit: 5}
  end

  def verify_master(%{tcp: tcp, config: %Config{master_key: key}}, _opts) do
    case TCP.send(tcp, key) do
      :ok ->
        {:ok, :verified}

      error ->
        error
    end
  end

  @impl true
  def handle_info({:tcp, _port, message}, state) do
    case Jason.decode(message) do
      {:ok, %{} = event} ->
        handle_tcp_event(event, state)
        {:noreply, state}

      _ ->
        {:noreply, state}
    end
  end

  def handle_info(_event, state) do
    {:noreply, state}
  end

  defp handle_tcp_event(%{"_op" => "add_avatar"} = params, state) do
    %RustlerLiveViewToy.Game.Avatar{}
    |> RustlerLiveViewToy.Game.Avatar.changeset(params)
    |> Ecto.Changeset.apply_changes()
    |> add_avatar(state)
  end

  defp handle_tcp_event(
         %{"_op" => "update_avatar", "key" => key, "value" => value},
         %{ets: ets} = state
       ) do
    case :ets.lookup(ets.avatars, key) do
      [{_id, record}] ->
        record
        |> RustlerLiveViewToy.Game.Avatar.changeset(%{message: value})
        |> Ecto.Changeset.apply_changes()
        |> add_avatar(state)

      _ ->
        nil
    end
  end

  defp handle_tcp_event(%{"_op" => "remove_avatar", "key" => key}, %{ets: ets} = state) do
    :ets.delete(ets.avatars, key)

    refresh_avatar(key, state)
  end

  defp handle_tcp_event(event, _state) do
    Logger.warn("#{inspect(__MODULE__)} - Unhandled TCP event: #{inspect(event)}")
  end

  defp add_avatar(%Avatar{key: key} = avatar, %{ets: ets} = state) do
    :ets.insert(ets.avatars, {key, avatar})

    refresh_avatar(key, state)
  end

  defp add_avatar(_data, _state), do: nil

  defp refresh_avatar(key, %{ets: ets}) do
    ets.players
    |> :ets.tab2list()
    |> Enum.map(fn {_id, port} ->
      send(port, {:cache, :avatars, {ets.avatars, key}})
    end)
  end
end
