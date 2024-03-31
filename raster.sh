#!/bin/bash -e

shopt -s nullglob
mkdir -p artifacts
set -x

# Generate emblem rasters
for path in artifacts/*.svg; do
	for height in 256 512 1024 2048 4096; do
		inkscape --export-type png -h ${height} -o artifacts/$(basename ${path%.svg})-${height}.png ${path}
	done
done

# Generate icon rasters
for path in icons/*.svg; do
	for height in 32 64 128 256 512 1024 2048; do
		inkscape --export-type png -h ${height} -o artifacts/$(basename ${path%.svg})-${height}.png ${path}
	done
done
