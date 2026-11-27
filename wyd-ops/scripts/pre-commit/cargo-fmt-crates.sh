#!/usr/bin/env bash
# Formata só os crates cujos arquivos o pre-commit passou como argumento.
# Formata no lugar (como os hooks de higiene): se mexer no arquivo, o hook falha
# e a pessoa re-adiciona o que foi corrigido.
#
# A raiz não tem `Cargo.toml` de workspace, então `cargo fmt` sem
# `--manifest-path` falha; e `cargo fmt --all` só existiria num workspace. O
# caminho de cada arquivo identifica o crate (`crates/<crate>/...`).
set -euo pipefail

crates=$(printf '%s\n' "$@" | sed -n 's|^crates/\([^/]*\)/.*|\1|p' | sort -u)

for crate in $crates; do
  [ -f "crates/$crate/Cargo.toml" ] || continue
  cargo fmt --manifest-path "crates/$crate/Cargo.toml"
done
