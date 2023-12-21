# SmartWallet Contract

SmartWallet is a Solidity smart contract that serves as a secure and flexible wallet with features such as owner management, allowance control, and transaction approvals from designated guardians.

## Table of Contents

- [Overview](#overview)
- [Key Features](#key-features)
- [Usage](#usage)
- [Functions](#functions)
- [License](#license)

## Overview

The SmartWallet contract is designed to provide enhanced security and functionality for managing funds. It allows the contract owner to propose a new owner, set allowances for specific addresses, deny sending capabilities, and execute transactions with proper validations.

## Key Features

- **Owner Management:** The contract owner can propose a new owner, and the transition requires confirmations from designated guardians.

- **Allowance Control:** The owner can set allowances for specific addresses, enabling controlled fund transfers.

- **Transaction Execution:** The contract supports secure and validated fund transfers to specified addresses.

## Usage

To use the SmartWallet contract, deploy it to the Ethereum blockchain and interact with it using Ethereum wallets or through decentralized applications (dApps).

## Functions

1. **proposeNewOwner**
   - Allows a guardian to propose a new owner for the SmartWallet contract.

2. **setAllowance**
   - Enables the owner to set allowances for specific addresses.

3. **denySending**
   - Allows the owner to deny sending capabilities to a specific address.

4. **transfer**
   - Executes a secure fund transfer to a specified address, subject to validations.

## License

This SmartWallet contract is licensed under the [MIT License](LICENSE). See the [LICENSE](LICENSE) file for more details.

