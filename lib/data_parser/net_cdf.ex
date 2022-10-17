defmodule DataParser.NetCDF do
  use Rustler, otp_app: :data_parser, crate: "dataparser_netcdf"

  # When your NIF is loaded, it will override this function.
  def open_file(_filename), do: :erlang.nif_error(:nif_not_loaded)
  def get_file_variables(_file_handle), do: :erlang.nif_error(:nif_not_loaded)
  def load_variable(_file_handle, _variable_name), do: :erlang.nif_error(:nif_not_loaded)
end
