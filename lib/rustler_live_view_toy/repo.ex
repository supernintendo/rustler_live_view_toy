defmodule RustlerLiveViewToy.Repo do
  use Ecto.Repo,
    otp_app: :rustler_live_view_toy,
    adapter: Ecto.Adapters.Postgres
end
