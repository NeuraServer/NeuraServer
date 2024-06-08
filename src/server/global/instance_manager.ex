defmodule ServerOps.InstanceManager do
  use GenServer

  @instance_count 5

  # Client API
  def start_link(_) do
    GenServer.start_link(__MODULE__, %{}, name: __MODULE__)
  end

  def start_instances do
    GenServer.call(__MODULE__, :start_instances)
  end

  def stop_instances do
    GenServer.call(__MODULE__, :stop_instances)
  end

  # Server Callbacks
  def init(_) do
    {:ok, %{instances: []}}
  end

  def handle_call(:start_instances, _from, state) do
    instances = Enum.map(1..@instance_count, fn id ->
      Task.async(fn -> start_instance(id) end)
    end)
    {:reply, :ok, %{state | instances: instances}}
  end

  def handle_call(:stop_instances, _from, state) do
    Enum.each(state.instances, fn instance ->
      Task.shutdown(instance, :brutal_kill)
    end)
    {:reply, :ok, %{state | instances: []}}
  end

  defp start_instance(id) do
    {:ok, _pid} = Plug.Cowboy.http(ServerOps.Router, [], port: 4000 + id)
    IO.puts("Started instance #{id} on port #{4000 + id}")
    :timer.sleep(:infinity)
  end
end
