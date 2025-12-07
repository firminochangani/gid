# gid (generate id)

[![ci](https://github.com/firminochangani/gid/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/firminochangani/gid/actions/workflows/ci.yml)

A CLI tool to generate unique identifiers.

## Examples

### Generating a UUID

```bash
$ gid uuid

0cadfc7c-1af9-45c9-a22c-6bf24f8bdec1
```

### Generating a ULID

```bash
$ gid ulid

01KB0X8GVBZVM7QBSW3RT7AJ70
```

## Generating multiple UUIDs

```bash
$ gid -c 10 uuid

b76597fe-794d-48e8-b43d-3545214783dc
10ebe09c-17a0-4f1e-9c39-fc600fa5d0a4
b79e5c08-8fe3-42a1-930b-4ae2e990a3b8
1d9dd2b4-d248-4b88-9a4a-cd6406178713
20d71877-46d6-48aa-9526-ef799e7d7cde
8a40e59a-7dbf-48a4-b072-2a3bd2d6f05c
e2480aa4-19cc-4a95-b90f-e07efa1a02fa
fe231d7d-93d7-4008-aff3-2564d1d5df54
e2d6e14c-f10c-4b97-a55b-40aa965b7ebc
5c13a7b8-338e-4e2e-8435-865a23d9b085
```

## Supported identifiers

- uuid
- ulid
- ksuid

## License

[GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE)