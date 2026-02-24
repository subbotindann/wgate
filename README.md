# winrun + waygate

linux win32 compatibility prototype.

## cli

- `winrun <file>`
- `winrun -d <file>`

## how it works

- `winrun` parses pe imports and selects only symbols implemented by `waygate`.
- `waygate` routes calls by dll domain (`kernel32`, `user32`).
- each dll module exports a `SUPPORTED` list and `dispatch` function.
- to add a new api, implement it in the target dll module, add the symbol to that module's `SUPPORTED`, then rebuild.

## build

```bash
cargo fmt --all
cargo build --workspace
```
