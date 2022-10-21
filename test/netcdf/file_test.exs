defmodule NetCDF.FileTest do
  use ExUnit.Case

  alias NetCDF.File

  @priv_dir :code.priv_dir(:netcdf)
  @filename Path.join(@priv_dir, "hello_world.nc")

  describe "open/1" do
    test "Loads data from existing file" do
      assert {:ok, %File{filename: @filename, resource: resource, variables: variables}} =
               File.open(@filename)

      assert is_reference(resource)
      assert variables == ~w(lat lon time temp)
    end

    test "errors on existing but invalid file" do
      assert {:error, {:netcdf_error, -51}} == File.open(Path.join(@priv_dir, "invalid.nc"))
    end

    test "errors on inexistent file" do
      assert {:error, {:netcdf_error, 2}} == File.open(Path.join(@priv_dir, "other.nc"))
    end
  end
end
