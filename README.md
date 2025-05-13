# todo

update asset data (during the stream) + make asset data immutable (after stream)
https://developers.metaplex.com/core/update


session keys 
https://docs.magicblock.gg/pages/tools/session-keys/integrating-sessions-in-your-program

attacks
https://github.com/coral-xyz/sealevel-attacks

# Build program and IDL

https://github.com/solana-foundation/anchor/issues/3662

```bash
anchor build --no-idl
```

```bash
RUSTUP_TOOLCHAIN=nightly-2025-04-01 anchor idl build -o target/idl/haus.json -t target/types/haus.ts
```

```bash
RUSTUP_TOOLCHAIN=nightly-2025-04-01 anchor build
```
