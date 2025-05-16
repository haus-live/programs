# Deployments

The program is currently deployed on devnet: https://explorer.solana.com/address/8SjSBampBM2asLdQeJoAZpxJxpcbBEGG5q9ADRCAFxr5?cluster=devnet


## Building

Need to use nightly build due to https://github.com/solana-foundation/anchor/issues/3662

```bash
avm use 0.30.1
```

```bash
RUSTUP_TOOLCHAIN=nightly-2025-04-01 anchor build
```

## Testing

```bash
RUSTUP_TOOLCHAIN=nightly-2025-04-01 anchor localnet
```

```bash
RUSTUP_TOOLCHAIN=nightly-2025-04-01 anchor run test
```


### Links

session keys 
https://docs.magicblock.gg/pages/tools/session-keys/integrating-sessions-in-your-program

attacks
https://github.com/coral-xyz/sealevel-attacks

# Build program and IDL (typescript client)

https://www.anchor-lang.com/docs/clients/typescript
