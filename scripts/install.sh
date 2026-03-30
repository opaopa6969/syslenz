#!/bin/sh
# syslenz installer
# Usage: curl -sSf https://raw.githubusercontent.com/opaopa6969/syslenz/main/scripts/install.sh | sh
#
# Environment variables:
#   SYSLENZ_VERSION   - Install a specific version (e.g. "v1.3.0" or "1.3.0")
#   SYSLENZ_INSTALL   - Override install directory (default: /usr/local/bin or ~/.local/bin)
#   SYSLENZ_MUSL      - Set to "1" to prefer musl binary on Linux x86_64

set -eu

REPO="opaopa6969/syslenz"
BINARY_NAME="syslenz"

# --- helpers ----------------------------------------------------------------

say() {
    printf '%s\n' "$1"
}

err() {
    say "error: $1" >&2
    say "エラー: $2" >&2
    exit 1
}

warn() {
    say "warning: $1" >&2
}

need_cmd() {
    if ! command -v "$1" > /dev/null 2>&1; then
        err "need '$1' (command not found)" "'$1' コマンドが見つかりません"
    fi
}

# --- platform detection -----------------------------------------------------

detect_os() {
    _os="$(uname -s)"
    case "$_os" in
        Linux)  echo "linux" ;;
        Darwin) echo "macos" ;;
        *)      err "unsupported OS: $_os" "未対応のOS: $_os" ;;
    esac
}

detect_arch() {
    _arch="$(uname -m)"
    case "$_arch" in
        x86_64 | amd64)     echo "x86_64" ;;
        aarch64 | arm64)    echo "aarch64" ;;
        *)                  err "unsupported architecture: $_arch" "未対応のアーキテクチャ: $_arch" ;;
    esac
}

# --- version resolution -----------------------------------------------------

get_latest_version() {
    # Use GitHub API to find the latest release tag.
    # Try curl first, then wget.
    _url="https://api.github.com/repos/${REPO}/releases/latest"
    if command -v curl > /dev/null 2>&1; then
        _response="$(curl -sSf "$_url" 2>/dev/null)" || {
            err "failed to fetch latest release from GitHub API" \
                "GitHub APIから最新リリースの取得に失敗しました"
        }
    elif command -v wget > /dev/null 2>&1; then
        _response="$(wget -qO- "$_url" 2>/dev/null)" || {
            err "failed to fetch latest release from GitHub API" \
                "GitHub APIから最新リリースの取得に失敗しました"
        }
    else
        err "need 'curl' or 'wget'" "'curl' または 'wget' が必要です"
    fi

    # Extract tag_name without jq (POSIX-compatible).
    _tag="$(printf '%s' "$_response" | grep '"tag_name"' | head -1 | sed 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/')"
    if [ -z "$_tag" ]; then
        err "could not determine latest version from GitHub API response" \
            "GitHub APIレスポンスから最新バージョンを特定できませんでした"
    fi
    echo "$_tag"
}

# --- download ---------------------------------------------------------------

download() {
    _url="$1"
    _output="$2"
    if command -v curl > /dev/null 2>&1; then
        curl -sSfL "$_url" -o "$_output"
    elif command -v wget > /dev/null 2>&1; then
        wget -qO "$_output" "$_url"
    else
        err "need 'curl' or 'wget'" "'curl' または 'wget' が必要です"
    fi
}

# --- checksum verification --------------------------------------------------

verify_checksum() {
    _file="$1"
    _expected_sha="$2"

    if command -v sha256sum > /dev/null 2>&1; then
        _actual="$(sha256sum "$_file" | cut -d ' ' -f 1)"
    elif command -v shasum > /dev/null 2>&1; then
        _actual="$(shasum -a 256 "$_file" | cut -d ' ' -f 1)"
    else
        warn "neither sha256sum nor shasum found; skipping checksum verification"
        warn "sha256sum/shasumが見つかりません。チェックサム検証をスキップします"
        return 0
    fi

    if [ "$_actual" != "$_expected_sha" ]; then
        err "checksum mismatch (expected ${_expected_sha}, got ${_actual})" \
            "チェックサム不一致 (期待値: ${_expected_sha}, 実測値: ${_actual})"
    fi
}

# --- main -------------------------------------------------------------------

main() {
    say ""
    say "  syslenz installer"
    say "  ================="
    say ""

    # Detect platform
    _os="$(detect_os)"
    _arch="$(detect_arch)"
    say "  Platform:  ${_os}/${_arch}"

    # Build the artifact name
    _artifact="${BINARY_NAME}-${_os}-${_arch}"
    if [ "$_os" = "linux" ] && [ "$_arch" = "x86_64" ] && [ "${SYSLENZ_MUSL:-0}" = "1" ]; then
        _artifact="${BINARY_NAME}-${_os}-${_arch}-musl"
        say "  Variant:   musl (static)"
    fi

    # Resolve version
    if [ -n "${SYSLENZ_VERSION:-}" ]; then
        _version="${SYSLENZ_VERSION}"
        # Normalise: add "v" prefix if missing
        case "$_version" in
            v*) ;;
            *)  _version="v${_version}" ;;
        esac
        say "  Version:   ${_version} (pinned)"
    else
        say "  Resolving latest version..."
        _version="$(get_latest_version)"
        say "  Version:   ${_version} (latest)"
    fi

    # Determine install directory
    if [ -n "${SYSLENZ_INSTALL:-}" ]; then
        _install_dir="${SYSLENZ_INSTALL}"
    elif [ -w /usr/local/bin ]; then
        _install_dir="/usr/local/bin"
    else
        _install_dir="${HOME}/.local/bin"
        mkdir -p "$_install_dir"
    fi
    say "  Install:   ${_install_dir}/${BINARY_NAME}"
    say ""

    # Require tar
    need_cmd tar

    # Build download URLs
    _base_url="https://github.com/${REPO}/releases/download/${_version}"
    _archive_url="${_base_url}/${_artifact}.tar.gz"
    _checksum_url="${_base_url}/${_artifact}.tar.gz.sha256"

    # Create a temporary directory (cleaned up on exit)
    _tmp_dir="$(mktemp -d 2>/dev/null || mktemp -d -t 'syslenz-install')"
    trap 'rm -rf "$_tmp_dir"' EXIT

    # Download archive
    say "  Downloading ${_artifact}.tar.gz ..."
    download "$_archive_url" "${_tmp_dir}/archive.tar.gz" || {
        err "failed to download ${_archive_url}" \
            "ダウンロードに失敗しました: ${_archive_url}"
    }

    # Download and verify checksum
    say "  Verifying checksum ..."
    if download "$_checksum_url" "${_tmp_dir}/checksum.sha256" 2>/dev/null; then
        _expected="$(cut -d ' ' -f 1 < "${_tmp_dir}/checksum.sha256")"
        verify_checksum "${_tmp_dir}/archive.tar.gz" "$_expected"
        say "  Checksum:  OK"
    else
        warn "checksum file not available; skipping verification"
    fi

    # Extract
    say "  Extracting ..."
    tar xzf "${_tmp_dir}/archive.tar.gz" -C "${_tmp_dir}"

    if [ ! -f "${_tmp_dir}/${BINARY_NAME}" ]; then
        err "binary '${BINARY_NAME}' not found in archive" \
            "アーカイブ内にバイナリ '${BINARY_NAME}' が見つかりません"
    fi

    # Install
    say "  Installing to ${_install_dir}/${BINARY_NAME} ..."
    chmod +x "${_tmp_dir}/${BINARY_NAME}"

    if [ -w "$_install_dir" ]; then
        mv "${_tmp_dir}/${BINARY_NAME}" "${_install_dir}/${BINARY_NAME}"
    else
        say "  Elevated permissions required. You may be prompted for your password."
        say "  管理者権限が必要です。パスワードの入力を求められる場合があります。"
        sudo mv "${_tmp_dir}/${BINARY_NAME}" "${_install_dir}/${BINARY_NAME}"
    fi

    # Verify
    if command -v "${_install_dir}/${BINARY_NAME}" > /dev/null 2>&1; then
        _installed_version="$("${_install_dir}/${BINARY_NAME}" --version 2>/dev/null || true)"
        say ""
        say "  Installed: ${_installed_version}"
    else
        say ""
        say "  Installed to ${_install_dir}/${BINARY_NAME}"

        # Check if install dir is in PATH
        case ":${PATH}:" in
            *":${_install_dir}:"*) ;;
            *)
                say ""
                say "  NOTE: ${_install_dir} is not in your PATH."
                say "  Add it with:"
                say ""
                say "    export PATH=\"${_install_dir}:\$PATH\""
                say ""
                say "  注意: ${_install_dir} がPATHに含まれていません。"
                say "  以下を実行してください:"
                say ""
                say "    export PATH=\"${_install_dir}:\$PATH\""
                ;;
        esac
    fi

    say ""
    say "  Done! Run 'syslenz' to get started."
    say "  完了！ 'syslenz' を実行して開始してください。"
    say ""
}

main "$@"
