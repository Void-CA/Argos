#!/bin/bash
echo "Compilando argos-cli..."
cargo build -p argos-cli --release

echo "Copiando binario a ~/.cargo/bin/argos"
cp target/release/argos-cli ~/.cargo/bin/argos

echo "¡Listo! Ejecuta: argos --help"
