# horizon-centered-crosshair

Port of [centered-crosshair](https://github.com/JustAlittleWolf/centered-crosshair)
(MIT (c) 2023 JustAlittleWolf) to Horizon's native-mod Rust API.

Vanilla draws the crosshair with integer blit coordinates, `x = (w - 15) / 2`
in gui-scaled pixels, where the gui-scaled size already lost its fraction to
`floor(windowWidth / guiScale)`. When the gui-scaled size is even the sprite
lands half a pixel off the true window centre. This port re-anchors the sprite
at the real centre computed from the raw window size in f32, quantised to
quarter pixels, and draws it through a subpixel textured-quad path with the
sprite's u/v nudged by a 1/32768 half-texel for clean edge sampling.

## Scope of this slice

The mod's entire logic is ported 1:1 in `runtime.rs` (10/10 tests green):

- `HudMixin` -> `centered_crosshair_draw`: quarter-pixel-quantised centre from
  the raw window size and gui scale, 15x15 sprite, crosshair pipeline.
- `GuiGraphicsExtractorMixin` / `DrawContextFloatDrawTexture` ->
  `subpixel_blit_sprite`: half-texel u/v nudge (u up, v down), colour `-1`
  (opaque white), zero-size early return, `createBounds` scissor logic.
- `SubpixelPositionedTexturedQuadGuiElementRenderState` ->
  `SubpixelTexturedQuad`: the four vertices in upstream `buildVertices` order
  and the bounds rectangle.
- Java `Math.round` semantics (`floor(x + 0.5)`) preserved via `java_round`.

## Known degradation

Horizon's `GuiDraw::Sprite` only carries integer x/y, so the actual draw of a
float-position HUD sprite has no API surface yet. The math is complete and
tested; the draw needs a Horizon hook (a float-position sprite blit path or a
crosshair-draw override). Filed upstream as an issue on horizon-rust/horizon.

## License

MIT, per upstream. See `LICENSE`.
