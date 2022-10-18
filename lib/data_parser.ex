defmodule DataParser do
  @moduledoc """
  Documentation for `DataParser`.
  """

  alias DataParser.NetCDF

  def netcdf_to_tensors(filename) do
    with {:ok, file} <- NetCDF.open_file(filename),
         {:ok, var_names} <- NetCDF.get_file_variables(file) do
      tensors =
        for var_name <- var_names, into: %{} do
          {:ok, values} = NetCDF.load_variable(file, var_name)
          {var_name, Nx.tensor(values)}
        end

      {:ok, tensors}
    end
  end
end
