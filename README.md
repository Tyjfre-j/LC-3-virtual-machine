# LC-3 Virtual Machine (Rust)

## Run

Put your LC-3 program images in `programs/` (for example `programs/2048.obj` and `programs/rogue.obj`).

```powershell
# build (once)
cargo build --release

# run via cargo (builds + runs)
cargo run --release -- 2048
cargo run --release -- rogue
```

You can also run by path:

```powershell
cargo run --release -- .\programs\2048.obj
```

Or run the built exe directly:

```powershell
.\target\release\lc-3-virtual-machine.exe .\programs\2048.obj
```

## References

- Tutorial: “Write your Own Virtual Machine” — Justin Meiners & Ryan Pendleton
  - `https://www.jmeiners.com/lc3-vm`
