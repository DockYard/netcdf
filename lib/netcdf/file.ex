defmodule NetCDF.File do
  @moduledoc """
  Represents a NetCDF File
  """
  defstruct [:resource, :filename, :variables]

  @doc """
  Returns a `__MODULE__` struct with all its fields
  """
  defdelegate open(filename), to: NetCDF.Native, as: :file_open_with_variables
end
