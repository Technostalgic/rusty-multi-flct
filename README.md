# Rusty Multi-FLCT

## Compiling

### Python bindings

prereqs:
* gcc
* gcc-c++
* uv

Install `uv` python package manager. It's the only one worth using. Then run:

```sh
$ cd bindings/python
$ uv venv && uv pip install maturin[patchelf] && uv run maturin develop
```

**NOTE - If you run into any issues:**
cfitsio is hard to compile because it usually tries to use the c++ compiler 
which doesn't work for it. so you need to set it to use an earlier c++ compiler
to avoid issues with using the reserved nullptr keyword. Run maturin develop 
with an environment that specifies a compatible compiler:

```sh
env CC="gcc" CFLAGS="-std=gnu17" uv run maturin develop
```