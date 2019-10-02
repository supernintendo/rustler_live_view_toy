defmodule RustlerLiveViewToyWeb.AudioLive do
  use Phoenix.LiveView

  def render(assigns) do
    ~L"""
    <span phx-hook="PlaySound"></span>
    """
  end
end
