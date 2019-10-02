defmodule RustlerLiveViewToyWeb.InputLive do
  use Phoenix.LiveView

  def render(assigns) do
    ~L"""
    <span phx-hook="MouseUp"></span>
    <span phx-hook="MouseDown"></span>
    <span phx-keyup="keyup" phx-target="window"></span>
    <span phx-keydown="keydown" phx-target="window"></span>
    """
  end
end
