# DistriAI-Core
This repository contains the core Solana program for the DistriAI.

## Development
### Dependencies
- rust version 1.77.2
- solana-cli 1.18.11
- anchor-cli 0.29.0

### Deployments
- devnet: 6yFTDdiS1W9T9yg6YejkwKggkEE4NYqdSSzVqQvuLn16

### Building Locally
Note: If you are running the build on an Apple computer with an M1 chip, please set the default rust toolchain to `stable-x86_64-apple-darwin`
```
rustup default stable-x86_64-apple-darwin
```

#### Compiling Program
```
anchor build
```

### Building in Solana Playground
1. Open https://beta.solpg.io/
1. Import from local file system

### Test in Solana Playground

![39F192EA-80DA-4EA1-9DD3-10873F9EFED2](https://github.com/distri-group/DistriAI-Core-Solana/assets/96568736/efd8fdd1-eb93-44ca-86d2-5c18486e7165)

1. Switch to `BUILD & DEPLOY`
2. Change program id.
3. Click `Build`
4. Switch to `Test`

![1713147722077](https://github.com/distri-group/DistriAI-Core/assets/122685398/b80c9548-f9b2-4fec-aff0-5f4db3e0cef9)      
