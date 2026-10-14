# Scientific Workflow: external tools fork

This is a fork of [Dingyi Sun's Scientific-Workflow](https://github.com/dingyisun0101/Scientific-Workflow).
It exposes a few existing components for use by the separate
[MPS Manager](https://github.com/wtrhacker/mps-manager) project.

## What changed

- Added `config::expand_parameters` as a public entry point to the existing
  parameter expander. Cartesian combinations, correlated cases, numeric ranges,
  ordering, and validation use the original implementation.
- Added the optional `external-tools` Rust feature, exposing the existing owned
  process groups and terminal components through `scientific_workflow::external`.
  Process cleanup can wait for an application's resource teardown, and explicit
  pause/resume also handles signals sent outside the application.
- Moved terminal restoration into the terminal lease, so an independently
  packaged viewer restores the terminal when it closes.

The existing Workflow runtime uses these same components. Its study grammar,
scientific execution behavior, recordings, and Python companion are preserved.

## What MPS Manager uses

MPS Manager uses the parameter expander, process-group ownership and cleanup,
terminal lease, command editor, status colors, and host resource sampler.
Its queue, NVIDIA MPS service handling, Python interface, and terminal browser
live in the **separate MPS Manager repository**. It does not use Workflow's
analysis pipeline or Python companion.

See [the extension notes](EXTERNAL_TOOLS.md) for the exported tools, and
[the original upstream README](UPSTREAM_README.md) for ordinary Workflow usage.

## Check the fork

```bash
cargo test --locked -p scientific-workflow
cargo test --locked -p scientific-workflow --features external-tools
```

Based on upstream v0.16.1, commit `c6e788eb4a8e2e8c52cd1a4260b7949fe8a95e2d`.
Licensed under [MIT](rust/LICENSE).
