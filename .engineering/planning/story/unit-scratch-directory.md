---
format: aep.planning-md/3
id: story:unit-scratch-directory
kind: story
status: draft
title: A unit's writable directory that lives as long as one instance (R118)
relations:
- decomposes: epic:architecture-deployment
revision: 1
---
## Outcome

A unit of a binding declares a writable directory at a mount path that lives as long as one
instance of the unit and holds nothing after a restart, distinct from a volume with a claim (R118).
Use: a service unit whose program writes run files under a fixed path while its configuration and
secret files are mounted read-only. `scratch: [{mount, size?}]` on a unit `packing.units` lists or
on a part under `parts:`; the bundle carries it per unit as `scratch`, sorted by `mount`, with no
volume claim. Design: `design/deployment.md` K21. Epic: `epic:architecture-deployment`.

## Scope

- `crates/agenticspec-sim/src/deploy/scratch.rs` (new): form, resolution, collisions,
  `check_units`.
- `crates/agenticspec-sim/src/deploy/binding.rs`, `packing.rs`: the key on a part and on a listed
  unit.
- `crates/agenticspec-sim/src/deploy/bundle.rs`: the unit's `scratch`, the findings at bundle and
  at check.
- `crates/agenticspec-sim/src/deploy/admit.rs`: refusals `unit_scratch_invalid`,
  `unit_mount_collision`.
- `crates/agenticspec-sim/src/deploy/fixtures.rs`: `config.rs` and `scratch.rs` as sources of the
  fixture bundles' digest.
- `crates/agenticspec-sim/tests/cases/deploy_scratch.rs`, `tests/cases/deploy/unit_mount_collision.yaml`,
  `tests/cases/deploy/unit_scratch_invalid.yaml`.
- `docs/deployment.md`, `design/deployment.md`, `CHANGELOG.md`, `AGENTS.md`,
  `bundles/sources.sha256`.

## Acceptance

- Given a binding with `scratch` on a listed unit and on parts, when `deploy bundle` runs, each
  unit carries its directories as `scratch` sorted by `mount`, its `claims.volumes` unchanged, and
  `deploy check` admits the bundle.
- Given a scratch directory at, inside or around a volume mount of its unit, or at the mount of
  another scratch directory of the unit, when `deploy bundle` runs, it refuses
  `unit_mount_collision`; a path that only starts like another (`/var/lib/flowx`) is admitted.
- Given a relative `mount`, a non-normal path, a missing `mount`, a non-text `size`, another key, a
  `scratch` that is no list, or `scratch` on a part in no unit, `deploy bundle` refuses
  `unit_scratch_invalid`.
- Given a written bundle whose unit's `scratch` or volume mount was changed to collide, or whose
  `scratch` was made relative or dropped, `deploy check` refuses `unit_mount_collision` or
  `unit_scratch_invalid`.
- Given a binding without `scratch`, the bundle is the same bytes as before (the fixture bundles
  rebuilt afresh compare equal).
- Given the test step, no test runs over 60 s and the step ends within 120 s.

## Not in this story

A medium for the directory (memory); a refusal of two volumes of one unit mounted at one path.
