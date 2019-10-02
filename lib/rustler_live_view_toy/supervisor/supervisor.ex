defmodule RustlerLiveViewToy.Supervisor do
  alias RustlerLiveViewToy.Supervisor.{Child, Config}

  def start(%Config{} = config) do
    config
    |> encode_children()
    |> start_supervisor()
  end

  def encode_children(%Config{child_defs: child_defs} = config) do
    %Config{config | children: Enum.map(child_defs, &encode_child/1)}
  end

  def encode_child(%Child{} = child) do
    %{
      id: child.id || child.module,
      start: {child.module, :start_link, child.start_args},
      type: child.type
    }
  end

  def start_supervisor(%Config{children: children} = config) do
    Supervisor.start_link(children, strategy: config.strategy, name: config.name)
  end
end
