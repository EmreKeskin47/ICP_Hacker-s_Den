# ICP Cross Chain Hacker's Den Content

## Content

-   **intro**: Slides about ICP
-   **examples/1-events**: This is the first example of an ICP canister implemented in Rust. It demonstrates essential techniques for managing state and making HTTP calls from within a canister.
-   **examples/2-dao**: implementation of a Decentralized Autonomous Organization (DAO) illustrating state management and the use of timers for executing repetitive tasks.
-   **examples/3-nft-container**: Last ICP canister example using Rust, demonstrating how to create and interact with NFTs in ICP

## ICP Glossary

-   **Canister**: Analogous to a smart contract in Ethereum Virtual Machine (EVM).
-   **Principal**: Similar to an address in EVM.
-   **Cycles**: Comparable to gas in EVM, used for transaction processing and canister operations.
-   **ABI**: In ICP, the Application Binary Interface is represented by Candid.

## Local Dev Setup

IC SDK is designed to work directly with Linux or macOS 12. If you are using Windows, you can use WSL1 or WSL2. Even though you can use both, WSL2 is recommended. To install the Internet Computer Software Development Kit, run:

```bash
`sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"`
```

Check that you have the DFINITY execution command-line interface (CLI) installed and the dfx executable is available in your PATH by running the following command:

```bash
`dfx --version`
```

-   **Note**: We will be using dfx version 0.15.1

## Local Development Setup

The Internet Computer Software Development Kit (IC SDK) is optimized for Linux and macOS 12 environments. Windows users can leverage WSL1 or WSL2, with WSL2 being the recommended option.

To install the IC SDK, execute:

```bash
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```

Verify the installation of the DFINITY execution command-line interface (CLI) and ensure the `dfx` executable is in your PATH:

```bash
dfx --version
```

**Note**: This repository uses dfx version 0.15.1.

## DFX Commands Guide

Here's a detailed guide to frequently used DFX commands:

-   `dfx start --clean --background`: Starts the local replica of the Internet Computer, cleaning any previous state and running in the background.
-   `dfx identity whoami`: Displays the current identity used by dfx.
-   `dfx identity use <identity_name>`: Switches to a different identity for subsequent dfx commands.
-   `dfx identity get-principal`: Retrieves the principal identifier of the current identity.
-   `dfx stop`: Stops the local Internet Computer replica.

### Troubleshooting: Stopping DFX Running in the Background

If you need to forcefully stop a dfx process running in the background:

1. Identify the process using:
    ```bash
    sudo lsof -i :4943
    ```
2. Terminate the process with:
    ```bash
    sudo kill <PID>
    ```

## Further Reading and Resources

-   [How it works?](https://internetcomputer.org/how-it-works/): A detailed explanation of the Internet Computer's architecture and principles.
-   [Identity management](https://internetcomputer.org/docs/current/developer-docs/setup/accounts)
-   [Rust ic_cdk](https://docs.rs/ic-cdk/latest/ic_cdk/): Documentation for the Rust Canister Development Kit (CDK).
-   [DIP721 NFT Standard](https://github.com/Psychedelic/DIP721/blob/develop/README.md): Comprehensive guide on the DIP721 NFT standard for the Internet Computer.
-   [Projects and Resources](https://github.com/dfinity/awesome-internet-computer#canister-development-kits-cdks): A collection of projects, tools, and resources for canister development on the Internet Computer.
