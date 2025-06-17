defmodule Zkvm do
  use Rustler,
    otp_app: :anoma_sdk,
    crate: :zkvm

  defmodule ForwarderCalldata do
    defstruct [:untrusted_forwarder, :input, :output]
  end

  defmodule ComplianceInstance do
    defstruct [:consumed_nullifier]
  end

  def testfunc(), do: :erlang.nif_error(:nif_not_loaded)
  def prove(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
  def verify(_proof_str), do: :erlang.nif_error(:nif_not_loaded)
end
