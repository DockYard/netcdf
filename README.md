
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
## Authors ##

* [Paulo Valente](https://github.com/polvalente)

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