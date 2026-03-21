# Substrate Dilithium Solochain

Modified Substrate solochain used to evaluate **post-quantum signatures (CRYSTALS-Dilithium / ML-DSA)**.

This node integrates Dilithium into the transaction signing and verification pipeline for benchmarking and comparison against classical schemes (sr25519, ECDSA).

## Purpose

This node is part of a larger FYP project evaluating the impact of post-quantum signatures on blockchain performance.

It is used together with a custom Subxt client to:
- execute transactions with different signature schemes
- measure latency, throughput, and verification cost
- analyse trade-offs between security and performance

## ⚠️ Origin

This project is based on the [Substrate](https://substrate.io/) solochain template from Parity Technologies.

The original template has been modified to support Dilithium (ML-DSA) signatures for experimental evaluation.

## Changes in this repository

Go to branch "dilithium" for an explanation of the changed 

## Orginal README

The orginal code and README can be found at: https://github.com/paritytech/polkadot-sdk-solochain-template 

## License

This project is based on the Substrate solochain template, released under the Unlicense.

Modifications and additions in this repository are provided under the MIT License.
