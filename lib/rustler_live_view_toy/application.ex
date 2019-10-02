defmodule RustlerLiveViewToy.Application do
  @moduledoc false

  use Application
  alias RustlerLiveViewToy.Supervisor.{Child, Config}

  @supervisor %Config{
    child_defs: [
      %Child{module: RustlerLiveViewToyWeb.Endpoint, type: :supervisor},
      # %Child{module: RustlerLiveViewToy.Repo},
      %Child{module: RustlerLiveViewToy.Engine}
    ],
    name: RustlerLiveViewToy.Supervisor
  }

  def start(_type, _args) do
    RustlerLiveViewToy.Supervisor.start(@supervisor)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  def config_change(changed, _new, removed) do
    RustlerLiveViewToyWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
