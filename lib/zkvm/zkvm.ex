defmodule Zkvm do
  use Rustler,
    otp_app: :local_domain,
    crate: :zkvm

  defmodule ForwarderCalldata do
    defstruct [:untrusted_forwarder, :input, :output]
  end

  def prove(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
  def verify(_proof_str), do: :erlang.nif_error(:nif_not_loaded)
end
