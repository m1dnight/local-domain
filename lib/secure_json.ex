defmodule LocalDomain.SecureJson do
  use Rustler,
    otp_app: :local_domain,
    crate: :host

  # When loading a NIF module, dummy clauses for all NIF function are required.
  # NIF dummies usually just error out when called when the NIF is not loaded, as that should never normally happen.
  def main(), do: :erlang.nif_error(:nif_not_loaded)

end
