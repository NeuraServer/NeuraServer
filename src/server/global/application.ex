defmodule ServerOps.Application do
  use Application

  def start(_type, _args) do
    children = [
      {ServerOps.InstanceManager, []}
    ]

    opts = [strategy: :one_for_one, name: ServerOps.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
