defmodule DataParser do
  @moduledoc """
  Documentation for `DataParser`.
  """

  alias DataParser.NetCDF

  @last_dim_coords ["lat", "lon", "latc", "lonc"]

  def netcdf_to_tensors(filename) do
    with {:ok, file} <- NetCDF.open_file(filename),
         {:ok, var_names} <- NetCDF.get_file_variables(file) do
      tensors =
        for var_name <- var_names, into: %{} do
          {:ok, {values, type_name}} = NetCDF.load_variable(file, var_name)
          {var_name, Nx.tensor(values, type: to_nx_type(type_name))}
        end

      var_coords_full =
        for var_name <- var_names do
          with {:ok, attrs} <- NetCDF.get_variable_attributes(file, var_name) do
            {var_name,
             Enum.find_value(attrs, fn
               {"coordinates", coords} -> String.split(coords, ~r/\s/, trim: true)
               _ -> nil
             end)}
          end
        end
        |> Map.new()

      var_coords =
        var_coords_full
        |> Enum.filter(fn {_k, v} -> v end)
        |> Map.new(fn {k, v} ->
          {k,
           Enum.split_with(v, &(Map.has_key?(var_coords_full, &1) and &1 not in @last_dim_coords))}
        end)

      data =
        for {var, tensor} <- tensors, into: %{} do
          {coords, last_dim_coords} = var_coords[var] || {nil, []}

          t =
            if not is_nil(coords) and coords != [] do
              shape =
                coords
                |> Enum.map(&Nx.size(tensors[&1]))
                |> List.to_tuple()
                |> Tuple.append(:auto)

              Nx.reshape(tensor, shape)
            else
              tensor
            end

          {var,
           %{
             "data" => t,
             "coords" => last_dim_coords
           }}
        end

      {:ok, data}
    end
  end

  defp to_nx_type("i8"), do: :s8
  defp to_nx_type("i16"), do: :s16
  defp to_nx_type("i32"), do: :s32
  defp to_nx_type("i64"), do: :s64
  defp to_nx_type("u8"), do: :u8
  defp to_nx_type("u16"), do: :u16
  defp to_nx_type("u32"), do: :u32
  defp to_nx_type("u64"), do: :u64
  defp to_nx_type("f32"), do: :f32
  defp to_nx_type("f64"), do: :f64
end
