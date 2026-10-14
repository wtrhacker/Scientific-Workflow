# Extensions for external applications

This fork preserves Scientific Workflow's existing study, runtime, recordings,
and Python companion. It makes a few existing infrastructure components reusable
by applications maintained in their own repositories.

`config::expand_parameters(&serde_json::Value)` exposes the existing parameter
expander without loading a study. Its Cartesian sweeps, correlated cases, numeric
ranges, ordering, validation, and literal-array semantics are unchanged.

The optional Rust feature `external-tools` exposes `scientific_workflow::external`:

- `spawn` and `ProcessTree` retain existing process-group ownership and cleanup.
  External adapters can defer ordinary cleanup until domain-specific resources
  have been released, then call `stop_after_teardown`. Explicit suspension works
  after stop/continue signals issued outside Workflow.
- `open_surface` and `TerminalLease` reuse terminal acquisition and restoration.
  Restoration belongs to the lease so a separately packaged viewer restores raw
  mode, mouse capture, cursor, and alternate-screen state on exit.
- `CommandInput`, `EditAction`, `TaskStatus`, `task_status_style`, `UsageMonitor`,
  and `UsageSnapshot` reuse the existing editor, palette, and host sampler.

There is no MPS daemon, MPS CLI, MPS queue, or additional Python package in this
repository. Those features belong to the independent `mps-manager` repository,
which consumes this opt-in API. No dependency on `mps-manager` is introduced here.

The ordinary Workflow runtime continues to use the same process and terminal
implementations. No study/configuration grammar or scientific execution contract
is changed. Run `cargo test --locked -p scientific-workflow` for existing behavior,
and add `--features external-tools` to verify the exported extension surface.

Upstream baseline: `c6e788eb4a8e2e8c52cd1a4260b7949fe8a95e2d` (v0.16.1).
