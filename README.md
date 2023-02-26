# Kujira Governance

Governance contracts for the Kujira Senate, forked from [cw-plus](https://github.com/CosmWasm/cw-plus). Please see that repository for a detailed breakdown.

These are broken down into two contracts: the CW3 Flex Multisig and the CW4 Group contract, as below.

CW3 Multisig:

- [`cw3-flex-multisig`](./contracts/cw3-flex-multisig) builds on cw3-fixed-multisig,
  with a more powerful implementation of the cw3 spec. It's a multisig contract
  backed by a cw4 (group) contract, which independently maintains the voter set.

- We add a `max_weight` and a `min_weight` to the contract config, to constrain the
  total amount of voting power that the membership can contain

- We add an `identity` field to `Member`, so that members of this voting set can be
  idenfiied by community members

CW4 Group:

- [`cw4-group`](./contracts/cw4-group) a basic implementation of the
  [cw4 spec](./packages/cw4/README.md). It handles elected membership, by admin or multisig.
  It fulfills all elements of the spec, including raw query lookups,
  and is designed to be used as a backing storage for [cw3 compliant contracts](./packages/cw3/README.md).

- We remove the requirement of membership to create proposals, instead replacing it with a deposit requirement

- We extend the proposal status to respoect a `Veto` result, subsequently burning the deposit if these conditions
  are met.

## Deployed Code

### Mainnet

- cw3: 83

- cw4: 80

### Testnet

- cw3: 1380

- cw4: 168: kujira1a5q689l9q777za8v078fgfqaqzhcm0myjy3kv549zfzrl6qv5drstrwk9s

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
