# Usage:
# Run `make` or `make all` to generate all combinations of OS=linux/mac and NIF_VERSION=2.15/2.16
# for the host architecture. Mostly useful for aarch64 because CI takes care of x86_64 releases.

VALID_NIF_VERSIONS := 2.15 2.16

VERSION_PATTERN := ^[0-9]+\.[0-9]+\.[0-9]+$$

# Set the VERSION environment variable using the contents of the VERSION file
VERSION := $(shell cat VERSION)

# Check that the VERSION variable matches the expected pattern
ifeq ($(shell echo $(VERSION) | grep -Eo '$(VERSION_PATTERN)'),$(VERSION))
  $(info VERSION is valid: '$(VERSION)')
else
  $(error VERSION is invalid: '$(VERSION)')
endif

ifeq ($(shell uname -s), Darwin)
TARGET_OS := apple-darwin
else
TARGET_OS := unknown-linux-gnu
endif

ifeq ($(shell uname -m), arm64)
# Processor is Mac arm
TARGET_ARCH := aarch64
else
# Processor is not Mac arm
TARGET_ARCH := x86_64
endif

NETCDF_SO_PREFIX := libex_netcdf-v$(VERSION)-nif-$(NIF_VERSION)-$(TARGET_ARCH)
NETCDF_SO := $(NETCDF_SO_PREFIX)-$(TARGET_OS).so
GNU_NETCDF_SO := $(NETCDF_SO_PREFIX)-unknown-linux-gnu.so

.PHONY: all
all:
	@echo "Building targets for NIF versions: $(VALID_NIF_VERSIONS) and arch: $(TARGET_ARCH)"
	@$(foreach nif_version,$(VALID_NIF_VERSIONS), \
		$(MAKE) gnu-docker-release NIF_VERSION=$(nif_version); \
		$(MAKE) host-release NIF_VERSION=$(nif_version); \
	)

.PHONY: validate_nif_env
validate_nif_env:
	@if [ -z "$$NIF_VERSION" ]; then \
        echo "NIF_VERSION is not set"; \
        exit 1; \
	fi
	@if [ -z "$(filter ${NIF_VERSION}, ${VALID_NIF_VERSIONS})" ]; then \
		echo "NIF_VERSION is set to ${NIF_VERSION}, but it should be one of: ${VALID_NIF_VERSIONS}"; \
		exit 1; \
	fi
	@echo "NIF_VERSION is set to ${NIF_VERSION} and is valid"

.PHONY: gnu-docker-release
gnu-docker-release: validate_nif_env
	docker build --tag netcdf-release --build-arg RUSTLER_NIF_VERSION=$(NIF_VERSION) -f Dockerfile.gnu-linux .
	docker run --mount type=bind,source="$(shell pwd)",target=/mnt/release netcdf-release cp /app/lib/netcdf-$(VERSION)/priv/native/libex_netcdf.so /mnt/release/$(GNU_NETCDF_SO)
	tar -czvf $(GNU_NETCDF_SO).tar.gz $(GNU_NETCDF_SO)
	rm $(GNU_NETCDF_SO)

.PHONY: host-release
host-release: validate_nif_env
	mix deps.get --only=prod
	RUSTLER_NIF_VERSION=$(NIF_VERSION) MIX_ENV=prod NETCDF_BUILD=true mix release --overwrite --force
	mv _build/prod/rel/netcdf/lib/netcdf-$(VERSION)/priv/native/libex_netcdf.so $(NETCDF_SO)
	tar -czvf $(NETCDF_SO).tar.gz $(NETCDF_SO)
	rm $(NETCDF_SO)