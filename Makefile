# examples:
# NIF=2.15 NETCDF_TARGET=aarch64-apple-darwin VERSION=0.2.0 make host-release
# NIF=2.15 NETCDF_TARGET=aarch64-unknown-linux-gnu VERSION=0.2.0 make gnu-docker-release

VALID_NETCDF_TARGETS_MAC := aarch64-apple-darwin x86_64-apple-darwin
VALID_NETCDF_TARGETS_GNU := aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu
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


BASE_NETCDF_TARGET_LIB := libex_netcdf-v$(VERSION)-nif-$(NIF_VERSION)-$(NETCDF_TARGET)
NETCDF_SO := $(BASE_NETCDF_TARGET_LIB).so

.PHONY: all
all: validate_nif_env check_netcdf_target_set
	@echo "All required env vars are set. Use one of the release rules to build your library"

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

.PHONY: check_netcdf_target_set
check_netcdf_target_set:
	@if [ -z "$$NETCDF_TARGET" ]; then \
        echo "NETCDF_TARGET is not set"; \
        exit 1; \
    fi

.PHONY: validate_gnu_env
validate_gnu_env: validate_nif_env check_netcdf_target_set
	@if [ -z "$(filter ${NETCDF_TARGET}, ${VALID_NETCDF_TARGETS_GNU})" ]; then \
		echo "Invalid NETCDF_TARGET: ${NETCDF_TARGET}. Valid options are: ${VALID_NETCDF_TARGETS_GNU}"; \
		exit 1; \
	fi
	@echo "NETCDF_TARGET is set to ${NETCDF_TARGET} and is valid"

.PHONY: validate_mac_env
validate_mac_env: validate_nif_env check_netcdf_target_set
	@if [ -z "$(filter ${NETCDF_TARGET}, ${VALID_NETCDF_TARGETS_MAC})" ]; then \
		echo "Invalid NETCDF_TARGET: ${NETCDF_TARGET}. Valid options are: ${VALID_NETCDF_TARGETS_MAC}"; \
		exit 1; \
	fi
	@echo "NETCDF_TARGET is set to ${NETCDF_TARGET} and is valid"

.PHONY: validate_host_env
ifeq ($(shell uname -s), Darwin)
validate_host_env: validate_mac_env
else
validate_host_env: validate_gnu_env
endif

.PHONY: gnu-docker-release
gnu-docker-release: validate_gnu_env
	docker build --tag netcdf-release --build-arg RUSTLER_NIF_VERSION=$(NIF_VERSION) -f Dockerfile.gnu-linux .
	docker run --mount type=bind,source="$(shell pwd)",target=/mnt/release netcdf-release cp /app/lib/netcdf-$(VERSION)/priv/native/libex_netcdf.so /mnt/release/$(NETCDF_SO)
	tar -czvf $(NETCDF_SO).tar.gz $(NETCDF_SO)
	rm $(NETCDF_SO)

.PHONY: host-release
host-release: validate_host_env
	mix deps.get --only=prod
	RUSTLER_NIF_VERSION=$(NIF_VERSION) MIX_ENV=prod NETCDF_BUILD=true mix release --overwrite --force
	mv _build/prod/rel/netcdf/lib/netcdf-$(VERSION)/priv/native/libex_netcdf.so $(NETCDF_SO)
	tar -czvf $(NETCDF_SO).tar.gz $(NETCDF_SO)
	rm $(NETCDF_SO)