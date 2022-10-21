# NetCDF

Elixir NIF Bindings for the Rust NetCDF library

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `netcdf` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:netcdf, "~> 0.0.1"}
  ]
end
```

### Dependencies

Before compiling, you also must ensure that the following dependencies are installed on your system:

- hdf5
- libnetcdf

On Ubuntu:

`apt install ibhdf5-serial-dev libnetcdf-dev`

On macOS:

`brew install netcdf-cxx`, which will also bring `hdf5` as a dependency

### Utilities

Although not necessary for the library to work, `ncks` is helpful for downloading `.nc` files in the correct way,
as well as reading their contents for header and format exploration.

On Ubuntu, you can install it using: `apt install nco`
On macOS, you can install it using: `brew install nco`
