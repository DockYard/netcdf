defmodule NetCDF.VariableTest do
  use ExUnit.Case

  alias NetCDF.File
  alias NetCDF.Variable

  @filename Path.join(:code.priv_dir(:netcdf), "hello_world.nc")

  describe "load/1" do
    test "loads existing variable" do
      {:ok, file} = File.open(@filename)

      assert {:ok, %Variable{name: "lat", value: [_ | _], type: :f32, attributes: attributes}} =
               Variable.load(file, "lat")

      assert attributes == %{"long_name" => "latitude", "units" => "degrees_north"}

      assert {:ok, %Variable{name: "lon", value: [_ | _], type: :f32, attributes: attributes}} =
               Variable.load(file, "lon")

      assert attributes == %{"long_name" => "longitude", "units" => "degrees_east"}

      assert {:ok, %Variable{name: "time", value: [_ | _], type: :f64, attributes: attributes}} =
               Variable.load(file, "time")

      assert attributes == %{"long_name" => "time", "units" => "hours since 1800-01-01"}

      assert {:ok, %Variable{name: "temp", value: [_ | _], type: :f64, attributes: attributes}} =
               Variable.load(file, "temp")

      assert attributes == %{"standard_name" => "air_temperature", "units" => "K"}
    end

    test "errors on inexistent variable" do
      {:ok, file} = File.open(@filename)

      assert {:error, :not_found} == Variable.load(file, "randomname")
    end

    test "errors on invalid file" do
      assert_raise ErlangError,
                   "Erlang error: \"Could not decode field :resource on %NetCDFFile{}\"",
                   fn ->
                     Variable.load(
                       %File{filename: "name", variables: ["var"], resource: make_ref()},
                       "lat"
                     )
                   end
    end

    test "loads string variable" do
      filename = Path.join(:code.priv_dir(:netcdf), "string_test.nc")
      {:ok, file} = File.open(filename)

      assert %File{filename: ^filename, resource: _, variables: ["string_var"]} = file

      assert {:ok,
              %Variable{
                name: "string_var",
                value: ~w(Hello world ! This is a test),
                type: :string,
                attributes: attributes
              }} = Variable.load(file, "string_var")

      assert %{} == attributes
    end
  end
end
