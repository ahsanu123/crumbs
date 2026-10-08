# fish_pond_rust

`no_std` port of `src/fish-body-simulation` using `embedded-graphics`.
The module layout mirrors the TypeScript source so it can be restructured
later without first untangling the original simulation math.

The library owns no allocator-backed collections and has no browser or clock
dependency. Create a `Pond`, then update and render whenever a frame should
advance:

```rust
let mut config = SceneConfig::new(display_width, display_height);
config.seed = embassy_time::Instant::now().as_ticks();
let mut pond = Pond::new(config);
pond.update(elapsed_ms);
pond.render(&mut display);
```

`step(&mut display, elapsed_ms)` remains as a convenience wrapper around those
two calls. Keeping them separate lets firmware skip drawing without freezing
visual interpolation or changing simulation state.

`elapsed_ms` drives the original fixed 60 Hz updates and ripple timers. Delays
and scheduling are entirely controlled by the caller. Fixed-step catch-up is
capped to prevent a slow display frame from aging entities through hundreds of
hidden updates. Randomness uses `fastrand`; the simulator seeds it from
`std::time::SystemTime`, while hardware can supply an Embassy time tick (or
another entropy source) through `SceneConfig::seed`.

The ESP32-S3 binary renders into a full RGB565 RAM framebuffer and flushes one
contiguous LCD region per frame. This avoids the primitive-level SPI address
window and transaction overhead of drawing directly to `mipidsi::Display`.

Spatial values are derived from `SceneConfig::width` and `height`. The original
960 x 640 scene is scale `1.0`; ripple growth, leave margins, and stroke widths
scale from the shorter screen dimension while fish and plant geometry retain
the TypeScript diagonal-based formulas.

Run the desktop simulator:

```sh
./run_sim.sh
```

The wrapper selects the stable host toolchain and disables the default Xtensa
feature/target automatically.

Mouse clicks add ripples and dash fish under the pointer. `A`, `D`, and `W`
chase toward pond edges, `R` toggles the rig, `V` removes one fish, and Escape
exits.

## ST7735S color diagnostic

Scene colors are specified as 8-bit RGB and converted to RGB565 component
ranges before drawing. `Rgb565::new` must not be called directly with CSS-style
0–255 components.

The `test_display` binary draws bars in this order:

`red, green, blue, white, cyan, magenta, yellow, black`

Use it to select the panel setting:

- Red and blue swapped: toggle `ColorOrder::Rgb` / `ColorOrder::Bgr`.
- Every color is complementary and white/black are swapped: toggle
  `ColorInversion::Normal` / `ColorInversion::Inverted`.
- Primaries are in the right positions but have a tint: the panel needs a
  different ST7735S gamma/init profile, not another RGB/BGR swap.
