defmodule NetCDF.File do
  @moduledoc """
  Represents a NetCDF File
  """
  defstruct [:resource, :filename, :variables]
  @type t :: %__MODULE__{resource: reference(), filename: String.t(), variables: [String.t()]}

  @doc """
  Returns a `__MODULE__` struct with all its fields
  """
  @spec open(filename :: String.t()) :: {:ok, t()} | {:error, term()}
  defdelegate open(filename), to: NetCDF.Native, as: :file_open_with_variables
end
