# Ray Tracer (Rust Path Tracer)

A full-featured path tracer built with Rust.

## Overview

This project is a physically based path tracer built with Rust. It delivers:

- **Ray‑tracing core:** BVH acceleration, parallel rendering, Russian roulette, and clean material system (Lambertian, Metal, Dielectric, Emissive).
- **Visual quality:** Depth‑of‑field, ACES tone mapping, sRGB encoding, and realistic glass effects.
- **Flexible scene format:** JSON configuration files supporting complex geometries, procedural parquet, mirrors, and custom lighting.
- **Developer‑friendly:** Comprehensive CLI, unit tests, and a clean, idiomatic Rust codebase.

## Quick Start

### Build

```bash
cargo build --release
```

### Run the default scene

```bash
cargo run --release
```

By default, the program automatically loads `scenes/output.json` and outputs `output.png`.

### Render with custom options

```bash
cargo run --release -- --scene scenes/output.json --width 1920 --height 1080 --samples 150 --output output.png
```

### Scene Configuration

The default scene file is located under `scenes/`:
- `output.json` – A detailed study interior featuring a colorful tiered bookshelf, an executive study desk with stacked hardcover books and a ceramic orb, an office chair, a floor lamp, herringbone parquet flooring, and a mirrored wardrobe reflecting the room and window daylight.

## Features

### Materials

- **Lambertian (diffuse)** – Real‑world BRDF with random hemisphere scattering.
- **Metal (specular with fuzz)** – Mirror‑like reflections with controlled roughness.
- **Dielectric (glass, refraction)** – Schlick Fresnel approximation + Snell’s law.
- **Emissive (lights)** – Self‑luminous surfaces for direct lighting.

### Geometry

- **Spheres** – Analytic ray‑sphere intersection, proper normals, and AoS + BVH support.
- **Quads** – Convenient rectangle primitive for walls/floor/ceiling.
- **Triangles** – General mesh building block.
- **BVH acceleration** – Median split on centroids, O(log n) traversal.

### Rendering Pipeline

- **Parallel pixel rendering** – Rayon per‑pixel parallelism with deterministic RNG.
- **Depth‑of‑field** – Thin‑lens camera model with configurable aperture & focus distance.
- **Russian roulette** – Intelligent path termination after bounce depth ~ 5.
- **ACES tone mapping** – Industry‑standard HDR compression with proper sRGB encoding.
- **Background gradient** – Sky‑blue horizon-to‑ground falloff.

### Scene System

- **JSON scene files** – Descriptive, human‑editable configuration using serde.
- **CLI overrides** – Width, height, samples, output path, seed, max depth.
- **Built‑in Cornell box** – Fully featured, production‑ready demo ready for presentation.

## How It Works (High‑Level)

1. **CLI parsing** – Clap handles overrides.
2. **Scene loading** – From JSON or the built‑in definition; deserialize materials and objects.
3. **BVH construction** – Build a bounding volume hierarchy over all scene objects.
4. **Camera setup** – Perspective pinhole or thin‑lens model with lens aperture and focus distance.
5. **Render loop:**
   - Parallel over each pixel with per‑pixel `SmallRng` for deterministic, thread‑local randomness.
   - Per pixel: jittered sampling → ray through lens (DoF) → trace through BVH.
   - Russian roulette after ~5 bounces to keep deep paths cheap.
   - Accumulate emitted + scattered + background contributions.
   - Apply ACES tone mapping and sRGB encode.
6. **Output** – PNG via `image` crate; image size matches configured resolution.

## Design Decisions

### Why Rust?

- **Performance:** zero‑cost abstractions, fine‑grained control over allocations.
- **Safety:** memory‑safe by default; optional `Send`/`Sync` for true parallelism.
- **Ecosystem:** Rich crate support (`rayon`, `serde`, `clap`, `image`).
- **Developer experience:** Excellent tooling, formatting, and type safety.

### Why BVH?

- **O(log n)** per ray vs. O(n) brute force.
- Median split on centroid AABBs is simple and effective.
- Recursive tree design mirrors classic ray‑tracer papers.

### Why ACES?

- Industry standard for HDR to SDR conversion.
- Produces visually pleasing results with minimal tuning.
- Better than ad‑hoc gamma curves for a wide range of scene luminances.

### Why deterministic per‑pixel RNG?

- Makes debugging reproducible.
- Allows perfect image comparison across runs.
- Avoids contention by seeding each pixel independently from its coordinates + global seed.

## Code Quality

- **Edition 2024** – Modern Rust features (const generics, pattern matching, etc.).
- **Documentation & comments** – Core algorithms, math, and design rationale.
- **Tests** – Unit tests for `Vec3` ops, ray‑sphere/Triangle hit, AABB, BVH, and tone mapping.
- **Clean abstractions:** `Material`, `Hittable`, `AABB`, `Camera` traits/structs.
- **No panics in release** – Graceful error handling and fallbacks.

## Contributing

- **The project is self‑contained** – No external dependencies beyond the Cargo ecosystem.
- **Feel free to extend:** Add new materials, geometries, or post‑processing passes.
- **Add test scenes** in `scenes/` and contribute back via pull requests.
- **Performance improvements** are welcome (SIMD, AoS cache layouts, etc.).