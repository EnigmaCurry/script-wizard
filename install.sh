#!/bin/bash

set -e

install_script_wizard() {
    ## This script is a self-container installer for script-wizard
    ## See https://github.com/EnigmaCurry/script-wizard
    stderr(){ echo "$@" >&2; }
    error(){ stderr "Error: $@"; }
    fault(){ test -n "$1" && error "$1"; stderr "Exiting."; exit 1; }

    local DEST_DIR
    DEST_DIR=$(realpath "${1:-${HOME}/.local/bin}")
    local DEST="${DEST_DIR}/script-wizard"

    check_deps() {
        local missing=""
        for dep in "$@"; do
            echo -n "Looking for ${dep} ... " >&2
            if ! command -v "${dep}" >/dev/null 2>&1; then
                echo "Missing!" >&2
                missing="${missing} ${dep}"
            else
                dep_path=$(command -v "${dep}" || true)
                echo "found ${dep_path}" >&2
            fi
        done
        [[ -n "${missing}" ]] && fault "Missing dependencies:${missing}"
        return 0
    }

    get_latest_version() {
        stderr "## Checking for latest version of script-wizard:"
        curl -sL https://api.github.com/repos/EnigmaCurry/script-wizard/releases/latest |
            jq -r ".tag_name" | sed 's/^v//'
    }

    check_os() {
        local supported_os=("$@")
        local current_os
        current_os=$(uname -s)
        if ! printf "%s\n" "${supported_os[@]}" | grep -Fxq "${current_os}"; then
            echo -e "\nError: Your OS (${current_os}) is not supported."
            echo "Supported: ${supported_os[*]}"
            exit 1
        fi
        echo "Found supported OS: ${current_os}"
    }

    check_os_architecture() {
        local os=$1; shift
        local supported_arch=("$@")
        local current_arch
        current_arch=$(uname -m)
        if [[ "$(uname -s)" == "${os}" ]]; then
            if ! printf "%s\n" "${supported_arch[@]}" | grep -Fxq "${current_arch}"; then
                echo -e "\nError: Your architecture (${current_arch}) is not supported."
                echo "Supported architectures: ${supported_arch[*]}"
                exit 1
            fi
            echo "Found supported architecture: ${current_arch}"
        fi
    }

    download() {
        local version="${1:-latest}"
        local os_arch="script-wizard-$(uname -s)-$(uname -m).tar.gz"
        local url="https://github.com/EnigmaCurry/script-wizard/releases"
        if [[ "${version}" == "latest" ]]; then
            url="${url}/latest/download/${os_arch}"
        else
            url="${url}/download/v${version}/${os_arch}"
        fi

        local tmp_dir
        tmp_dir=$(mktemp -d)
        cd "${tmp_dir}"
        echo "## Downloading ..."
        curl -LO "${url}"
        tar xfvz "${os_arch}"
        mkdir -p "${DEST_DIR}"
        mv script-wizard "${DEST}"
    }

    check_deps jq curl
    check_os Linux
    check_os_architecture Linux x86_64 aarch64

    local latest_release
    latest_release=$(get_latest_version)
    echo "Latest version: ${latest_release}"

    if [[ -f "${DEST}" ]]; then
        echo "script-wizard already installed at ${DEST}"
        local installed_version
        installed_version=$("${DEST}" --version | cut -d ' ' -f 2)
        echo "Installed version: ${installed_version}"
        if [[ "${installed_version}" == "${latest_release}" ]]; then
            echo "You already have the latest version installed."
            return 0
        fi
        echo "Updating to latest version..."
    fi

    echo "## Installing script-wizard..."
    download "${latest_release}"
    echo "Installed new script-wizard version: $(${DEST} --version | cut -d ' ' -f 2) (${DEST})"
}

install_script_wizard "$@"
