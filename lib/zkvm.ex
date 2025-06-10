defmodule Zkvm do
  use Rustler,
    otp_app: :local_domain,
    crate: :zkvm

  defmodule Action do
    defstruct [:compliance_units]
  end

  # When loading a NIF module, dummy clauses for all NIF function are required.
  # NIF dummies usually just error out when called when the NIF is not loaded, as that should never normally happen.
  def add(_x, _y), do: :erlang.nif_error(:nif_not_loaded)

  def mkstruct(), do: :erlang.nif_error(:nif_not_loaded)
end
