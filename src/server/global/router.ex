defmodule ServerOps.Router do
  use Plug.Router

  plug :match
  plug :dispatch

  get "/health_check" do
    send_resp(conn, 200, "OK")
  end

  match _ do
    send_resp(conn, 404, "Not Found")
  end
end
