# Deployments

The program is currently deployed on devnet: https://explorer.solana.com/address/GZtbVznhmHTqn6PbiSN6PdJNPBboMW5gkCYszq9caNQ1?cluster=devnet


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

## Verification

```bash
RUSTUP_TOOLCHAIN=nightly-2025-04-01 anchor build --verifiable
```

```bash
RUSTUP_TOOLCHAIN=nightly-2025-04-01 anchor deploy --verifiable
```

```bash
RUSTUP_TOOLCHAIN=nightly-2025-04-01 anchor idl init --provider.cluster devnet --filepath target/idl/haus.json GZtbVznhmHTqn6PbiSN6PdJNPBboMW5gkCYszq9caNQ1
```

```bash
RUSTUP_TOOLCHAIN=nightly-2025-04-01 anchor verify --env RUSTUP_TOOLCHAIN=nightly-2025-04-01 -p haus GZtbVznhmHTqn6PbiSN6PdJNPBboMW5gkCYszq9caNQ1 --skip-build --provider.cluster devnet
```
