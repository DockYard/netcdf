
# NetCDF

Elixir NIF Bindings for the Rust NetCDF library

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `netcdf` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:netcdf, "~> 0.2.0"}
  ]
end
```

### Dependencies

For usage, you don't need any dependencies installed.
The library uses `:rustler_precompiled` to fetch compiled versions for the following targets:

- x86_64-apple-darwin
- x86_64-unknown-linux-gnu
- aarch64-apple-darwin
- aarch64-unknown-linux-gnu

If you want to compile locally or if you are running on an unsupported target,
you must set `NETCDF_BUILD=true` in your environment, and follow the instructions below.

Before compiling, you also must ensure that the following dependencies are installed on your system:

- hdf5@1.12.1
- libnetcdf@4.8.1

On Ubuntu:

`apt install libhdf5-serial-dev libnetcdf-dev`

On macOS:

`brew install netcdf-cxx`, which will also bring `hdf5` as a dependency

Conda can also be used to install the dependencies instead:

```shell
conda install -y -c conda-forge libnetcdf=4.8.1 hdf5=1.12.1
echo "HDF5_DIR=${CONDA_PREFIX}"
echo "NETCDF_DIR=${CONDA_PREFIX}" >> $GITHUB_ENV
echo "RUSTFLAGS=-C link-args=-Wl,-rpath,$CONDA_PREFIX/lib" >> $GITHUB_ENV
```

### Utilities

Although not necessary for the library to work, `ncks` is helpful for downloading `.nc` files in the correct way,
as well as reading their contents for header and format exploration.

On Ubuntu, you can install it using: `apt install nco`
On macOS, you can install it using: `brew install nco`

## Usage

A NetCDF file can be loaded into Elixir with the following command:

```elixir
filename = # Elixir string with the path to the NetCDF file
{:ok, file} = NetCDF.File.open(filename)
```

We can inspect its variables in the struct's `:variables` field.
Finally, variables can be loaded as follows:

```elixir
# loading the first variable from the file
variable_name = hd(file.variables)
{:ok, variable} = NetCDF.Variable.load(file, variable_name)
```

## Authors ##

- [Paulo Valente](https://github.com/polvalente)

[We are very thankful for the many contributors](https://github.com/dockyard/netcdf/graphs/contributors)

## Versioning ##

This library follows [Semantic Versioning](https://semver.org)

## Looking for help with your Elixir project? ##

[At DockYard we are ready to help you build your next Elixir project](https://dockyard.com/phoenix-consulting). We have a unique expertise
in Elixir and Phoenix development that is unmatched. [Get in touch!](https://dockyard.com/contact/hire-us)

At DockYard we love Elixir! You can [read our Elixir blog posts](https://dockyard.com/blog/categories/elixir)

## Legal ##

[DockYard](https://dockyard.com/), Inc. © 2022

[@DockYard](https://twitter.com/DockYard)

[Licensed under the MIT license](https://www.opensource.org/licenses/mit-license.php)