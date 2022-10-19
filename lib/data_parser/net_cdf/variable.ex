defmodule DataParser.NetCDF.Variable do
  @moduledoc """
  Variable access functions
  """
  defstruct [:name, :value, :type, :attributes]

  @type variable_type ::
          :i8 | :i16 | :i32 | :i64 | :u8 | :u16 | :u32 | :u64 | :f32 | :f64 | :non_numeric
  @type attribute_value :: atom() | number() | String.t() | [number()] | [String.t()]

  @type t :: %__MODULE__{
          name: String.t(),
          type: variable_type(),
          attributes: [{String.t(), attribute_value()}]
        }

  @doc """
  Returns a `__MODULE__` struct with the variable's values and associated metadata.
  """
  @spec load(file :: DataParser.NetCDF.File.t(), variable_name :: String.t()) ::
          {:ok, t()} | {:error, any()}
  defdelegate load(file, variable_name), to: DataParser.NetCDF.Native, as: :variable_load
end
