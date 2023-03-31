defmodule NetCDF.Native do
  @moduledoc false
  use RustlerPrecompiled,
    otp_app: :netcdf,
    crate: "ex_netcdf",
    base_url: Application.compile_env!(:netcdf, :base_url),
    version: Application.compile_env!(:netcdf, :version)


  # netcdf::file
  def file_open(_filename), do: :erlang.nif_error(:nif_not_loaded)
  def file_open_with_variables(_filename), do: :erlang.nif_error(:nif_not_loaded)
  def file_variables(_file_handle), do: :erlang.nif_error(:nif_not_loaded)

  # netcdf::variable
  def variable_load(_file_handle, _variable_name), do: :erlang.nif_error(:nif_not_loaded)
  def variable_values(_file_handle, _variable_name), do: :erlang.nif_error(:nif_not_loaded)

  def variable_attributes(_file_handle, _variable_name),
    do: :erlang.nif_error(:nif_not_loaded)
end
