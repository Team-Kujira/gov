# Kujira Governance

Governance contracts for the Kujira blockchain, forked from [cw-plus](https://github.com/CosmWasm/cw-plus). Please see that repository for a detailed breakdown.

These are broken down into two contracts: the CW3 Flex Multisig and the CW4 Group contract, as below.

The only changes made is to enforce a max and min weight for all members, to ensure a quality voting set, see https://github.com/Team-Kujira/gov/commit/696dd6655be9e45379dad533560e37cdd7c8e10e

CW3 Multisig:

- [`cw3-flex-multisig`](./contracts/cw3-flex-multisig) builds on cw3-fixed-multisig,
  with a more powerful implementation of the cw3 spec. It's a multisig contract
  backed by a cw4 (group) contract, which independently maintains the voter set.

CW4 Group:

- [`cw4-group`](./contracts/cw4-group) a basic implementation of the
  [cw4 spec](./packages/cw4/README.md). It handles elected membership, by admin or multisig.
  It fulfills all elements of the spec, including raw query lookups,
  and is designed to be used as a backing storage for [cw3 compliant contracts](./packages/cw3/README.md).

## Deployed Code

### Testnet

- cw4: 166

## Licenses

This repo contains two license, [Apache 2.0](./LICENSE-APACHE) and
[AGPL 3.0](./LICENSE-AGPL.md). All crates in this repo may be licensed
as one or the other. Please check the `NOTICE` in each crate or the
relevant `Cargo.toml` file for clarity.

All _specifications_ will always be Apache-2.0. All contracts that are
meant to be _building blocks_ will also be Apache-2.0. This is along
the lines of Open Zeppelin or other public references.

Contracts that are "ready to deploy" may be licensed under AGPL 3.0 to
encourage anyone using them to contribute back any improvements they
make. This is common practice for actual projects running on Ethereum,
like Uniswap or Maker DAO.
