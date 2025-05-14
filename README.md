# todo

update asset data (during the stream) + make asset data immutable (after stream)
https://developers.metaplex.com/core/update


session keys 
https://docs.magicblock.gg/pages/tools/session-keys/integrating-sessions-in-your-program

attacks
https://github.com/coral-xyz/sealevel-attacks

# Build program and IDL (typescript client)

https://www.anchor-lang.com/docs/clients/typescript


```bash
avm use 0.30.1
```

```bash
RUSTUP_TOOLCHAIN=nightly-2025-04-01 anchor build
```

https://github.com/solana-foundation/anchor/issues/3662

# Testing

```bash
RUSTUP_TOOLCHAIN=nightly-2025-04-01 anchor localnet
```

## Metaplex Core Asset

```bash
solana program dump -u m CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d tests/metaplex_core_asset_program.so
```

## Metaplex Token Metadata


[test.validator]
bind_address = "0.0.0.0"
url = "http://solana.fm"
ledger = ".anchor/test-ledger"
rpc_port = 8899

[[test.validator.clone]]
address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"

[[test.validator.clone]]
address = "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"

# genesis

[[test.genesis]]
address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
program = "tests/metaplex_token_metadata_program.so"

[[test.genesis]]
address = "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7"
program = "tests/metaplex_core_asset_program.so"
