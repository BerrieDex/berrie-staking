# Berrie Staking Contract

![Solana Verified](https://img.shields.io/badge/Solana-Verified-green?style=for-the-badge&logo=solana)
![Build Status](https://img.shields.io/badge/Build-Reproducible-blue?style=for-the-badge)
![Security Audit](https://img.shields.io/badge/Security-Audited-orange?style=for-the-badge)

# Berrie Staking Contract

## 🔐 Program Verification & Security

### Verification Status

**Program ID**: `8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr`  
**Network**: Solana Mainnet Beta  
**Verification Status**: ✅ **Verified** ([Solscan](https://solscan.io/account/8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr))

### Real-Time Verification Links

| Service | Status | Link |
| --- | --- | --- |
| **Solana Explorer** | 🔍 View Program | [explorer.solana.com](https://explorer.solana.com/address/8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr) |
| **Solscan** | 📊 Analytics | [solscan.io](https://solscan.io/account/8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr) |
| **Solscan Program Verification** | ⏳ PENDING | [Verification Status](https://solscan.io/account/8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr) |

### Verification Details

*   **Program ID**: `8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr`
*   **Network**: Solana Mainnet Beta
*   **Verification Status**: ✅ **Fully Verified with Anchor**
*   **Git Commit Hash**: `be7ef5d8e2247ab9a76a4eca0965af83b373bdac`
*   **Verified Date**: December 7, 2025

## 🏗️ Reproducible Build Instructions

### Prerequisites

```shell
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor
npm install -g @coral-xyz/anchor-cli
```

### Anchor Build & Verification Process

The program has been built and verified using Anchor with the following process:

```shell
# Clone the repository
git clone https://github.com/BerrieDex/berrie-staking.git
cd berrie-staking

# Checkout the specific verified commit
git checkout be7ef5d8e2247ab9a76a4eca0965af83b373bdac

# Build with verifiable flag
anchor build --verifiable

# Deploy with verifiable flag
anchor deploy --verifiable

# Initialize IDL
anchor idl init -f target/idl/berrie_staking.json 8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr

# Verify the program
anchor verify -p berrie_staking 8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr
```

### Verification Results

The anchor verify command confirmed successful verification:

```
Copying out the build artifacts
Successfully copied 364kB to /home/ubuntu/rebuild/berrie-staking/target/verifiable/berrie_staking.so
Cleaning up the docker target directory
Removing the docker container
anchor-program
Extracting the IDL
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running unittests src/lib.rs (/home/ubuntu/rebuild/berrie-staking/target/debug/deps/berrie_staking-2b339a169248f0a5)
Writing the IDL file
Writing the .ts file
Build success
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (/home/ubuntu/rebuild/berrie-staking/target/debug/deps/berrie_staking-2b339a169248f0a5)
8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr is verified.
```

### Docker Verification

For maximum reproducibility, the program is built using Docker to ensure verification integrity. The verification process uses Docker containers to match the exact build environment.

# 🔐 Program Verification

This Solana program has been **verified** using the OtterVerify verification system. The verification ensures that the deployed program matches exactly with the source code in this repository.

## ✅ Verification Status

- **Program ID**: `8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr`
- **Verification Status**: ✅ **VERIFIED**
- **Verification Date**: July 23, 2025
- **Verified Commit**: [`edcc9ee`](https://github.com/BerrieDex/berrie-staking/commit/edcc9ee511537393338a3a44b3660219483d02bf)
- **Program Hash**: `9cd16376359ebb876cefe800ca049319e24827d004dcc9b993fe533e42782119`

## 🔍 Verification Proof

### On-Chain Verification Data
The verification is permanently stored on the Solana blockchain and can be independently verified:

- **Verification PDA**: [`8wuvPnsPTniMZnAN26qfEwgwFhU7BQHp7i2iPs5iE5mN`](https://explorer.solana.com/address/8wuvPnsPTniMZnAN26qfEwgwFhU7BQHp7i2iPs5iE5mN/anchor-account)
- **Verification Transaction**: [`2w5ZYcfrwD7Q44NHcUH66okVTg1SgNEMwJjTmAYXuNh8QHEzxFDV91Pu5bzz8JGjSnZ6n5MJ1HXoR6XzTWbRSqZU`](https://explorer.solana.com/tx/2w5ZYcfrwD7Q44NHcUH66okVTg1SgNEMwJjTmAYXuNh8QHEzxFDV91Pu5bzz8JGjSnZ6n5MJ1HXoR6XzTWbRSqZU)

### Verification Details
The verification PDA contains the following metadata:
- **Repository URL**: `https://github.com/BerrieDex/berrie-staking.git`
- **Commit Hash**: `edcc9ee511537393338a3a44b3660219483d02bf`
- **Library Name**: `berrie_staking`
- **Build Arguments**: `["--library-name", "berrie_staking"]`
- **Verification Tool Version**: `0.4.9`
- **Deploy Slot**: `352743460`

## 🛠️ Independent Verification

You can independently verify this program using the following methods:

### Method 1: Using solana-verify CLI

```bash
# Install solana-verify
cargo install solana-verify

# Check verification PDA
solana-verify get-program-pda \
  --program-id 8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr \
  -u https://api.mainnet-beta.solana.com

# Get program hash
solana-verify get-program-hash \
  8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr \
  -u https://api.mainnet-beta.solana.com

# Verify from repository (reproduces the verification)
solana-verify verify-from-repo \
  -u https://api.mainnet-beta.solana.com \
  --program-id 8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr \
  --commit-hash edcc9ee511537393338a3a44b3660219483d02bf \
  --library-name berrie_staking \
  https://github.com/BerrieDex/berrie-staking.git
```

### Method 2: Manual Build Verification

```bash
# Clone the repository
git clone https://github.com/BerrieDex/berrie-staking.git
cd berrie-staking

# Checkout the verified commit
git checkout edcc9ee511537393338a3a44b3660219483d02bf

# Build using the same method as verification
anchor build --verifiable

# Compare the hash of your build with the on-chain program
# The hash should match: 9cd16376359ebb876cefe800ca049319e24827d004dcc9b993fe533e42782119
```

### Method 3: Check Verification Account Data

```bash
# View the verification account directly
solana account 8wuvPnsPTniMZnAN26qfEwgwFhU7BQHp7i2iPs5iE5mN \
  -u https://api.mainnet-beta.solana.com
```

## 🔗 Explorer Links

- **Program Account**: [Solana Explorer](https://explorer.solana.com/address/8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr) | [Solscan](https://solscan.io/account/8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr)
- **Verification PDA**: [Solana Explorer](https://explorer.solana.com/address/8wuvPnsPTniMZnAN26qfEwgwFhU7BQHp7i2iPs5iE5mN/anchor-account)
- **Verification Transaction**: [Solana Explorer](https://explorer.solana.com/tx/2w5ZYcfrwD7Q44NHcUH66okVTg1SgNEMwJjTmAYXuNh8QHEzxFDV91Pu5bzz8JGjSnZ6n5MJ1HXoR6XzTWbRSqZU)

## 📋 What This Verification Guarantees

✅ **Source Code Authenticity**: The deployed program was built from the exact source code in this repository  
✅ **Build Reproducibility**: Anyone can rebuild the program and get the same binary hash  
✅ **Transparency**: All build parameters and metadata are publicly available on-chain  
✅ **Immutability**: The verification data is permanently stored on the Solana blockchain  
✅ **Independent Verification**: Third parties can verify the program without trusting our claims  

## 🔧 Build Information

- **Anchor Version**: `0.31.1`
- **Solana Version**: `v2.1.16`
- **Build Method**: Docker-based verifiable build
- **Rust Toolchain**: Solana's official toolchain

## 📚 Learn More

- [Solana Program Verification Guide](https://solana.com/developers/guides/advanced/verified-builds)
- [OtterVerify Documentation](https://github.com/Ellipsis-Labs/solana-verifiable-build)
- [Anchor Verifiable Builds](https://www.anchor-lang.com/docs/verifiable-builds)

---

*This verification was completed on July 23, 2025, and the verification data is permanently stored on the Solana blockchain. The verification ensures that users can trust that the deployed program matches the open-source code in this repository.*


## 🛡️ Security & Auditing

### Security Measures

*   ✅ **Verified Build**: Reproducible builds ensure deployed code matches source
*   ✅ **Open Source**: All code is publicly auditable
*   ✅ **Multi-Sig Authority**: Program authority secured by multi-signature wallet
*   ✅ **Security.txt**: Vulnerability disclosure process documented
*   ✅ **Immutable Deployment**: Program authority can be verified on-chain
*   ✅ **Anchor Framework**: Built with Anchor for enhanced security

### Security Audit Status

| Component | Status | Details |
| --- | --- | --- |
| **Smart Contract Logic** | ✅ Verified | Core staking mechanisms audited |
| **Access Controls** | ✅ Verified | Admin functions properly restricted |
| **Economic Model** | ✅ Verified | Tokenomics and reward calculations |
| **Integration Security** | ✅ Verified | Safe interaction patterns |

### Vulnerability Disclosure

We take security seriously. If you discover a vulnerability:

1.  **DO NOT** create a public issue
2.  Email security details to: `team@berr.ie`
3.  Include detailed reproduction steps
4.  Allow 90 days for responsible disclosure

See our [Privacy Policy](https://berrie.gitbook.io/berrie/privacy-policy) for complete details.

## 📊 Program Information

### Deployment Details

| Property | Value |
| --- | --- |
| **Program ID** | `8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr` |
| **Upgrade Authority** | `3hiQADryzHeV6gQa8gojLV5EHNAKXdujtTX2u8evVh1Z` (Multi-Sig) |
| **Network** | Solana Mainnet Beta |
| **Verification Status** | ✅ Fully Verified with Anchor |
| **Git Commit** | `be7ef5d8e2247ab9a76a4eca0965af83b373bdac` |

### Multi-Signature Security

The program upgrade authority is controlled by a multi-signature wallet for enhanced security:

*   **Multi-Sig Address**: `3hiQADryzHeV6gQa8gojLV5EHNAKXdujtTX2u8evVh1Z`
*   **Security Model**: Requires multiple signatures for any program upgrades
*   **Transparency**: All upgrade proposals and signatures are publicly visible on-chain
*   **View Multi-Sig**: [Solana Explorer](https://explorer.solana.com/address/3hiQADryzHeV6gQa8gojLV5EHNAKXdujtTX2u8evVh1Z)

### Network Information

*   **Network**: Solana Mainnet Beta
*   **RPC Endpoint**: `https://api.mainnet-beta.solana.com`
*   **Cluster**: `mainnet-beta`
*   **Commitment Level**: `confirmed`

## 🔍 Verification Monitoring

### Automated Status Checking

Monitor verification status programmatically:

```shell
# Check program deployment status
solana program show 8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr -u mainnet-beta

# Check multi-sig authority status
solana account 3hiQADryzHeV6gQa8gojLV5EHNAKXdujtTX2u8evVh1Z -u mainnet-beta

# Verify with Anchor
anchor verify -p berrie_staking 8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr
```

### Verification Timeline

| Date | Event | Details |
| --- | --- | --- |
| **December 7, 2025** | Program Deployed | Verified program deployed to mainnet |
| **December 7, 2025** | Anchor Verification Completed | Full verification completed with Anchor |
| **December 7, 2025** | Authority Transferred to Multi-Sig | `3hiQADryzHeV6gQa8gojLV5EHNAKXdujtTX2u8evVh1Z` |

## 🤝 Community Trust & Transparency

### Open Development

*   **Public Repository**: All development happens in the open
*   **Commit History**: Full development history available
*   **Issue Tracking**: Community can report bugs and request features
*   **Code Reviews**: All changes reviewed before deployment

### Third-Party Verification

Multiple independent verification services confirm program authenticity:

*   **Anchor Framework**: Primary verification method
*   **Solana Explorer**: Official Solana verification display
*   **Solscan**: Independent blockchain explorer verification

### Contact & Support

*   **Website**: [berr.ie](https://berr.ie/)
*   **Documentation**: [berrie.gitbook.io/berrie](https://berrie.gitbook.io/berrie)
*   **Privacy Policy**: [berrie.gitbook.io/berrie/privacy-policy](https://berrie.gitbook.io/berrie/privacy-policy)
*   **General Inquiries**: `team@berr.ie`
*   **Security Issues**: `team@berr.ie`
*   **Technical Support**: [GitHub Issues](https://github.com/BerrieDex/berrie-staking/issues)

### Community Channels

*   **Twitter**: [@BerrieOrg](https://x.com/BerrieOrg)
*   **Discord**: [Join Community](https://discord.gg/fHemmWRMyh)
*   **Telegram**: [BerrieFarm](https://t.me/BerrieFarm)

## 🔧 Development & Testing

### Build Environment

The program is built using a deterministic, reproducible environment to ensure verification integrity:

```yaml
Environment Specifications:
  - Solana Version: Latest Stable
  - Anchor Framework: Latest Stable
  - Rust Version: Latest Stable
  - Build Target: bpf-unknown-unknown
  - Verification Method: Anchor Verifiable Build
```

### Build Process

The verification process ensures that the deployed program matches the source code:

1. **Git Checkout**: Specific commit hash `be7ef5d8e2247ab9a76a4eca0965af83b373bdac`
2. **Anchor Build**: `anchor build --verifiable`
3. **Anchor Deploy**: `anchor deploy --verifiable`
4. **IDL Initialization**: `anchor idl init -f target/idl/berrie_staking.json`
5. **Verification**: `anchor verify -p berrie_staking`

## 📈 Program Analytics & Monitoring

### Real-Time Metrics

Monitor program usage and health through these dashboards:

*   **Solana Beach**: [Program Analytics](https://solanabeach.io/address/8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr)
*   **Solscan**: [Transaction History](https://solscan.io/account/8Hcd5Kmi47JhgNr1GVYfAbMyXfwYo5T7UUYhGeEu8qPr)
*   **Step Finance**: [DeFi Analytics](https://app.step.finance/)

## 📋 Compliance & Legal

### Regulatory Compliance

*   **Open Source License**: MIT License for maximum transparency
*   **Data Privacy**: No personal data collection or storage
*   **Jurisdictional Compliance**: Designed for global accessibility
*   **AML/KYC**: Decentralized design requires no KYC

## 📞 Support & Community

### Getting Help

1.  **Documentation**: Check [berrie.gitbook.io/berrie](https://berrie.gitbook.io/berrie) first
2.  **GitHub Issues**: Report bugs or request features
3.  **Discord Community**: Real-time support and discussion
4.  **Email Support**: Direct contact for complex issues at `team@berr.ie`

### Contributing

We welcome contributions from the community:

1.  **Fork** the repository
2.  **Create** a feature branch
3.  **Commit** your changes
4.  **Push** to the branch
5.  **Create** a Pull Request

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## 📄 License & Attribution

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

**Built with ❤️ by the BerrieDex Team**

_Last Updated: July 12, 2025_

