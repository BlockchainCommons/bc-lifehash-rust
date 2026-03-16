# Blockchain Commons LifeHash for Rust

<!--Guidelines: https://github.com/BlockchainCommons/secure-template/wiki -->

### _by Wolf McNally and Christopher Allen_

---

## Introduction

`bc-lifehash` is a method of hash visualization based on Conway's Game of Life that creates beautiful icons that are deterministic, yet distinct and unique given the input data.

The basic concept is to take a SHA-256 hash of the input data (which can be any data including another hash) and then use the 256-bit digest as a 16x16 pixel "seed" for running the cellular automata known as [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life). After the pattern becomes stable (or begins repeating) the resulting history is used to compile a grayscale image of all the states from the first to last generation. Using Game of Life provides visual structure to the resulting image, even though it was seeded with entropy. Some bits of the initial hash are then used to deterministically apply symmetry and color to the icon to add beauty and quick recognizability.

This is the first-party Rust implementation. It produces byte-identical output to the [C++ reference implementation](https://github.com/BlockchainCommons/bc-lifehash), validated against 35 test vectors covering all five versions.

See a LifeHash demo at [LifeHash.info](https://lifehash.info/).

## Getting Started

```toml
[dependencies]
bc-lifehash = "0.1.0"
```

## Versions

Five LifeHash versions are supported:

- **Version1** / **Version2** — 16x16 grid, up to 150 generations.
- **Detailed** — 32x32 grid, up to 300 generations, richer color gradients.
- **Fiducial** — 32x32, designed for use as fiducial markers.
- **GrayscaleFiducial** — Same as Fiducial but rendered in grayscale.

## Usage

```rust
// Generate a LifeHash image from a UTF-8 string
let image = bc_lifehash::make_from_utf8("Hello", bc_lifehash::Version::Version2, 1, false);
assert_eq!(image.width, 32);
assert_eq!(image.height, 32);
// image.colors contains packed RGB bytes (width * height * 3)
```

You can also generate from raw bytes or a pre-computed 32-byte SHA-256 digest:

```rust
let image = bc_lifehash::make_from_data(&[0x01, 0x02, 0x03], bc_lifehash::Version::Detailed, 1, false);

// Or from a digest directly (must be exactly 32 bytes)
let digest = [0u8; 32];
let image = bc_lifehash::make_from_digest(&digest, bc_lifehash::Version::Fiducial, 1, true);
// With has_alpha=true, image.colors contains RGBA bytes (width * height * 4)
```

## Implementations

| Type | Name | Language | Note |
|------|------|----------|------|
| Reference | [bc-lifehash](https://github.com/BlockchainCommons/bc-lifehash) | C++/C |
| First-Party | [bc-lifehash-rust](https://github.com/BlockchainCommons/bc-lifehash-rust) | Rust | This crate
| Third-Party | [lifehash-rs](https://github.com/GalactechsLLC/lifehash-rs) | Rust | [Galactechs LLC](https://github.com/GalactechsLLC)
| Third-Party | [bc-lifehash-python](https://github.com/BlockchainCommons/bc-lifehash-python) | Python | [Cramium](https://cramiumsolutions.com/)
| Third-Party | [toucan](https://github.com/sparrowwallet/toucan) | Java | [Sparrow Wallet](https://sparrowwallet.com/)
| Reference | [LifeHash](https://github.com/BlockchainCommons/LifeHash) | Swift |

## Version History

- **0.1.0, March 16, 2026**
  - Initial release of bc-lifehash crate.
  - LifeHash visual hashing algorithm based on Conway's Game of Life.
  - Supports five versions: Version1, Version2, Detailed, Fiducial, GrayscaleFiducial.
  - Deterministic SHA-256-seeded cellular automata with symmetry and color transforms.
  - Configurable module size and optional alpha channel output.
  - Test vectors and PNG generation tests.
  - Byte-identical output to the C++ reference implementation.

## Status - Community Review

`bc-lifehash` is currently in a community review stage. We would appreciate your consideration and/or testing of the libraries. Obviously, let us know if you find any mistakes or problems. But also let us know if the API meets your needs, if the functionality is easy to use, if the usage of Rust feels properly standardized, and if the library solves any problems you are encountering when doing this kind of coding. Also let us know how it could be improved and what else you'd need for this to be just right for your usage. Comments can be posted [to the Gordian Developer Community](https://github.com/BlockchainCommons/Gordian-Developer-Community/discussions/116).

Because this library is still in a community review stage, it should not be used for production tasks until it has had further testing and auditing.

See [Blockchain Commons' Development Phases](https://github.com/BlockchainCommons/Community/blob/master/release-path.md).

## Financial Support

`bc-lifehash` is a project of [Blockchain Commons](https://www.blockchaincommons.com/). We are proudly a "not-for-profit" social benefit corporation committed to open source & open development. Our work is funded entirely by donations and collaborative partnerships with people like you. Every contribution will be spent on building open tools, technologies, and techniques that sustain and advance blockchain and internet security infrastructure and promote an open web.

To financially support further development of `bc-lifehash` and other projects, please consider becoming a Patron of Blockchain Commons through ongoing monthly patronage as a [GitHub Sponsor](https://github.com/sponsors/BlockchainCommons). You can also support Blockchain Commons with bitcoins at our [BTCPay Server](https://btcpay.blockchaincommons.com/).

## Contributing

We encourage public contributions through issues and pull requests! Please review [CONTRIBUTING.md](./CONTRIBUTING.md) for details on our development process. All contributions to this repository require a GPG signed [Contributor License Agreement](./CLA.md).

### Discussions

The best place to talk about Blockchain Commons and its projects is in our GitHub Discussions areas.

[**Gordian Developer Community**](https://github.com/BlockchainCommons/Gordian-Developer-Community/discussions). For standards and open-source developers who want to talk about interoperable wallet specifications, please use the Discussions area of the [Gordian Developer Community repo](https://github.com/BlockchainCommons/Gordian-Developer-Community/discussions). This is where you talk about Gordian specifications such as [Gordian Envelope](https://github.com/BlockchainCommons/Gordian/tree/master/Envelope#articles), [bc-shamir](https://github.com/BlockchainCommons/bc-shamir), [Sharded Secret Key Reconstruction](https://github.com/BlockchainCommons/bc-sskr), and [bc-ur](https://github.com/BlockchainCommons/bc-ur) as well as the larger [Gordian Architecture](https://github.com/BlockchainCommons/Gordian/blob/master/Docs/Overview-Architecture.md), its [Principles](https://github.com/BlockchainCommons/Gordian#gordian-principles) of independence, privacy, resilience, and openness, and its macro-architectural ideas such as functional partition (including airgapping, the original name of this community).

[**Gordian User Community**](https://github.com/BlockchainCommons/Gordian/discussions). For users of the Gordian reference apps, including [Gordian Coordinator](https://github.com/BlockchainCommons/iOS-GordianCoordinator), [Gordian Seed Tool](https://github.com/BlockchainCommons/GordianSeedTool-iOS), [Gordian Server](https://github.com/BlockchainCommons/GordianServer-macOS), [Gordian Wallet](https://github.com/BlockchainCommons/GordianWallet-iOS), and [SpotBit](https://github.com/BlockchainCommons/spotbit) as well as our whole series of [CLI apps](https://github.com/BlockchainCommons/Gordian/blob/master/Docs/Overview-Apps.md#cli-apps). This is a place to talk about bug reports and feature requests as well as to explore how our reference apps embody the [Gordian Principles](https://github.com/BlockchainCommons/Gordian#gordian-principles).

[**Blockchain Commons Discussions**](https://github.com/BlockchainCommons/Community/discussions). For developers, interns, and patrons of Blockchain Commons, please use the discussions area of the [Community repo](https://github.com/BlockchainCommons/Community) to talk about general Blockchain Commons issues, the intern program, or topics other than those covered by the [Gordian Developer Community](https://github.com/BlockchainCommons/Gordian-Developer-Community/discussions) or the
[Gordian User Community](https://github.com/BlockchainCommons/Gordian/discussions).

### Other Questions & Problems

As an open-source, open-development community, Blockchain Commons does not have the resources to provide direct support of our projects. Please consider the discussions area as a locale where you might get answers to questions. Alternatively, please use this repository's [issues](./issues) feature. Unfortunately, we can not make any promises on response time.

If your company requires support to use our projects, please feel free to contact us directly about options. We may be able to offer you a contract for support from one of our contributors, or we might be able to point you to another entity who can offer the contractual support that you need.

### Credits

The following people directly contributed to this repository. You can add your name here by getting involved. The first step is learning how to contribute from our [CONTRIBUTING.md](./CONTRIBUTING.md) documentation.

| Name              | Role                     | Github                                           | Email                                 | GPG Fingerprint                                    |
| ----------------- | ------------------------ | ------------------------------------------------ | ------------------------------------- | -------------------------------------------------- |
| Christopher Allen | Principal Architect      | [@ChristopherA](https://github.com/ChristopherA) | \<ChristopherA@LifeWithAlacrity.com\> | FDFE 14A5 4ECB 30FC 5D22 74EF F8D3 6C91 3574 05ED  |
| Wolf McNally      | Lead Researcher/Engineer | [@WolfMcNally](https://github.com/wolfmcnally)   | \<Wolf@WolfMcNally.com\>              | 9436 52EE 3844 1760 C3DC  3536 4B6C 2FCF 8947 80AE |

## Responsible Disclosure

We want to keep all of our software safe for everyone. If you have discovered a security vulnerability, we appreciate your help in disclosing it to us in a responsible manner. We are unfortunately not able to offer bug bounties at this time.

We do ask that you offer us good faith and use best efforts not to leak information or harm any user, their data, or our developer community. Please give us a reasonable amount of time to fix the issue before you publish it. Do not defraud our users or us in the process of discovery. We promise not to bring legal action against researchers who point out a problem provided they do their best to follow the these guidelines.

### Reporting a Vulnerability

Please report suspected security vulnerabilities in private via email to ChristopherA@BlockchainCommons.com (do not use this email for support). Please do NOT create publicly viewable issues for suspected security vulnerabilities.

The following keys may be used to communicate sensitive information to developers:

| Name              | Fingerprint                                       |
| ----------------- | ------------------------------------------------- |
| Christopher Allen | FDFE 14A5 4ECB 30FC 5D22 74EF F8D3 6C91 3574 05ED |

You can import a key by running the following command with that individual's fingerprint: `gpg --recv-keys "<fingerprint>"` Ensure that you put quotes around fingerprints that contain spaces.
