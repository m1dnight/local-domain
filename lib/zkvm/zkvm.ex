defmodule Zkvm do
  use Rustler,
    otp_app: :local_domain,
    crate: :zkvm

  defmodule LogicProof do
    defstruct [:receipt, :verifying_key]
  end

  defmodule ForwarderCalldata do
    defstruct [:untrusted_forwarder, :input, :output]
  end

  def testfunc(), do: :erlang.nif_error(:nif_not_loaded)
end
