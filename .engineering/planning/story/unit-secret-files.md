---
format: aep.planning-md/3
id: story:unit-secret-files
kind: story
status: draft
title: A secret as a read-only file of a unit (R120)
relations:
- decomposes: epic:architecture-deployment
- informed_by: story:unit-scratch-directory
revision: 1
---
## Outcome

A binding gives a unit a secret as a read-only file (R120): `secret_files: [{secret: <name>,
key?: <key>, path: <absolute path>}]` on a unit `packing.units` lists or on a part under `parts:`,
the two places of `env` and `args`. The bundle carries the entries in the unit's `config` beside
`env` and `args`, sorted by `path`, by the secret's name, never a value. Design:
`design/deployment.md` K22. Epic: `epic:architecture-deployment`.

## Scope

- `crates/agenticspec-sim/src/deploy/config.rs`: the key, its form, its resolution into
  `config.secret_files`, the carried form at check.
- `crates/agenticspec-sim/src/deploy/grants.rs`: the secret of an entry through the unit's
  declared secrets (`secret_value`, `secret_undeclared`), at bundle and at check.
- `crates/agenticspec-sim/src/deploy/scratch.rs`: a secret file's path is one of the unit's own
  mounts, held with the scratch directories of `story:unit-scratch-directory` against each other
  and the unit's volume mounts (`unit_mount_collision`).
- `crates/agenticspec-sim/src/deploy/binding.rs`, `packing.rs`: the key on a part and on a listed
  unit.
- `crates/agenticspec-sim/tests/cases/deploy_secret_files.rs`.
- `docs/deployment.md`, `design/deployment.md`, `CHANGELOG.md`, `AGENTS.md`,
  `bundles/sources.sha256`.

## Acceptance

- Given a binding with `secret_files` on a listed unit and on a part whose secret a part of the
  unit declares, when `deploy bundle` runs, the unit's `config` carries `secret_files` sorted by
  `path` beside `env`, and `deploy check` admits the bundle.
- Given a secret file whose secret no part of the unit declares, `deploy bundle` refuses
  `secret_undeclared`; given one whose secret is no secret name, `secret_value`, without echoing
  it.
- Given a relative or non-normal `path`, a missing `secret` or `path`, a `key` that is not a key,
  another key, or a `secret_files` that is no list, `deploy bundle` refuses `unit_config_invalid`.
- Given two secret files at one path, or a secret file inside a volume mount or a scratch
  directory of its unit, `deploy bundle` refuses `unit_mount_collision`.
- Given a written bundle whose secret file was moved into a volume mount, given an undeclared
  secret, or made relative, `deploy check` refuses `unit_mount_collision`, `secret_undeclared` or
  `unit_config_invalid`.
- Given a binding without `secret_files`, the bundle is the same bytes as before.
- Given the test step, no test runs over 60 s and the step ends within 120 s.

## Not in this story

A file mode or owner for the file; a secret file of a UI unit.
