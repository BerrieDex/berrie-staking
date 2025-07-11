# Berrie Staking Contract

![Solana Verified](https://img.shields.io/badge/Solana-Verified-green?style=for-the-badge&logo=solana)
![Build Status](https://img.shields.io/badge/Build-Reproducible-blue?style=for-the-badge)
![Security Audit](https://img.shields.io/badge/Security-Audited-orange?style=for-the-badge)

# Berrie Staking Contract

## 🔐 Program Verification & Security

### Verification Status

**Program ID**: `4YXrP2NwHpCW7Vqwdmf7XVgnV7Fi345u9gLGW9ucuPW7`  
**Network**: Solana Mainnet Beta  
**Verification Status**: ✅ **Verified** ([Check Status](https://solscan.io/account/4YXrP2NwHpCW7Vqwdmf7XVgnV7Fi345u9gLGW9ucuPW7))

### Real-Time Verification Links

| Service | Status | Link |
| --- | --- | --- |
| **Solana Explorer** | 🔍 View Program | [explorer.solana.com](https://explorer.solana.com/address/4YXrP2NwHpCW7Vqwdmf7XVgnV7Fi345u9gLGW9ucuPW7) |
| **OSEC Verification** | ✅ Verified | [verify.osec.io](https://verify.osec.io/status/4YXrP2NwHpCW7Vqwdmf7XVgnV7Fi345u9gLGW9ucuPW7) |
| **Solscan** | 📊 Analytics | [solscan.io](https://solscan.io/account/4YXrP2NwHpCW7Vqwdmf7XVgnV7Fi345u9gLGW9ucuPW7) |

### Verification Details

*   **Program ID**: `4YXrP2NwHpCW7Vqwdmf7XVgnV7Fi345u9gLGW9ucuPW7`
*   **Network**: Solana Mainnet Beta
*   **Verification Status**: ✅ **Fully Verified**
*   **Verified Date**: July 11, 2025

## 🏗️ Reproducible Build Instructions

### Prerequisites

```shell
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor
npm install -g @coral-xyz/anchor-cli

# Install solana-verify CLI
cargo install solana-verify --version 0.4.9
```

### Build Verification

You can independently verify this program matches the deployed version:

```shell
# Clone the repository
git clone https://github.com/BerrieDex/berrie-staking.git
cd berrie-staking

# Build with verification
anchor build --verifiable

# Verify against deployed program
solana-verify verify-from-repo \
  --program-id 4YXrP2NwHpCW7Vqwdmf7XVgnV7Fi345u9gLGW9ucuPW7 \
  --library-name berrie_staking \
  https://github.com/BerrieDex/berrie-staking
```

### Docker Verification

For maximum reproducibility, use the official Solana Docker image:

```shell
# Set Docker resource limits (required for successful build)
export SVB_DOCKER_MEMORY_LIMIT=2g
export SVB_DOCKER_CPU_LIMIT=2

# Pull the official Solana verifiable build image
docker pull solanafoundation/solana-verifiable-build:latest

# Run verification build
docker run --rm -v $(pwd):/build solanafoundation/solana-verifiable-build:latest \
  bash -c "cd /build && anchor build --verifiable"
```

## 🛡️ Security & Auditing

### Security Measures

*   ✅ **Verified Build**: Reproducible builds ensure deployed code matches source
*   ✅ **Open Source**: All code is publicly auditable
*   ✅ **Multi-Sig Authority**: Program authority secured by multi-signature wallet
*   ✅ **Security.txt**: Vulnerability disclosure process documented
*   ✅ **Immutable Deployment**: Program authority can be verified on-chain

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
| **Program ID** | `4YXrP2NwHpCW7Vqwdmf7XVgnV7Fi345u9gLGW9ucuPW7` |
| **Upgrade Authority** | `3hiQADryzHeV6gQa8gojLV5EHNAKXdujtTX2u8evVh1Z` (Multi-Sig) |
| **Network** | Solana Mainnet Beta |
| **Verification Status** | ✅ Fully Verified |

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
# Check OSEC verification status
curl "https://verify.osec.io/status/4YXrP2NwHpCW7Vqwdmf7XVgnV7Fi345u9gLGW9ucuPW7"

# Check program deployment status
solana program show 4YXrP2NwHpCW7Vqwdmf7XVgnV7Fi345u9gLGW9ucuPW7 -u mainnet-beta

# Check multi-sig authority status
solana account 3hiQADryzHeV6gQa8gojLV5EHNAKXdujtTX2u8evVh1Z -u mainnet-beta
```

### Verification Timeline

| Date | Event | Details |
| --- | --- | --- |
| **July 11, 2025** | Program Deployed | Verified program deployed to mainnet |
| **July 11, 2025** | OSEC Verification Completed | Full verification completed |
| **July 11, 2025** | Authority Transferred to Multi-Sig | `3hiQADryzHeV6gQa8gojLV5EHNAKXdujtTX2u8evVh1Z` |

## 🤝 Community Trust & Transparency

### Open Development

*   **Public Repository**: All development happens in the open
*   **Commit History**: Full development history available
*   **Issue Tracking**: Community can report bugs and request features
*   **Code Reviews**: All changes reviewed before deployment

### Third-Party Verification

Multiple independent verification services confirm program authenticity:

*   **OSEC (OtterSec)**: Primary verification service
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
  - Docker Image: solanafoundation/solana-verifiable-build
  - Build Target: bpf-unknown-unknown
```

## 📈 Program Analytics & Monitoring

### Real-Time Metrics

Monitor program usage and health through these dashboards:

*   **Solana Beach**: [Program Analytics](https://solanabeach.io/address/4YXrP2NwHpCW7Vqwdmf7XVgnV7Fi345u9gLGW9ucuPW7)
*   **Solscan**: [Transaction History](https://solscan.io/account/4YXrP2NwHpCW7Vqwdmf7XVgnV7Fi345u9gLGW9ucuPW7)
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

_Last Updated: July 11, 2025_

