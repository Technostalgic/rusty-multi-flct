# Rusty Multi-FLCT

## Compiling

### Python bindings

Install `uv` python package manager. It's the only one worth using. Then run:

```sh
$ cd bindings/python
$ uv venv && uv pip install maturin[patchelf] && uv run maturin develop
```