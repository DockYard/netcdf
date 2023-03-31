defmodule NetCDF.Native do
  @moduledoc false

  mode = if Mix.env() in [:dev, :test], do: :debug, else: :release
  force_build = mode == :debug or System.get_env("NETCDF_BUILD") in ["1", "true"]

  mix_config = Mix.Project.config()
  version = mix_config[:version]
  github_url = mix_config[:package][:links]["GitHub"]

  use RustlerPrecompiled,
    otp_app: :netcdf,
    crate: "ex_netcdf",
    base_url: "#{github_url}/releases/download/v#{version}",
    version: version,
    force_build: force_build,
    mode: mode

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
