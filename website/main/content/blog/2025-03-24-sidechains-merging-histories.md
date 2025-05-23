---
title: "Thinking formally in terms of sidechains"
date: 2025-03-24
draft: false
description: Leveraging a formal framework for the construction of sidechains as a base for our design. 
tags: [ status-update, consensus ]
authors: [ adlrocha ]
---

Overall, I am really happy with the progress I've made this week. I've been mainly focused on unravelling one of the
papers that I mentioned in my last update, [Proof-of-Stake Sidechains](https://eprint.iacr.org/2018/1239.pdf). While is
true that we don't want our system to have anything to do with PoS, and there is no 1:1 matching of the concepts from
the paper with what we are trying to build, the paper presents a framework that can come pretty handy to evaluate the
correctness of our designs. This paper is from 2018, and after a first pass the first thing that I did is to check if
there were any follow-up papers that built upon the abstractions of this paper. I came
across [Proof-of-Work Sidechains](https://eprint.iacr.org/2018/1048.pdf) from the same authors, but without a doubt, the
most complete proposal is the one that I started with. Let's dive right into it.

<!--more-->

## Thinking in terms of sidechains.

What I liked the most about this work is that it presents a formal framework to reason about the interaction between
different blockchain networks through the construction of sidechains. The paper focuses on PoS sidechains, but the
concepts and the security model is general enough to be easily adaptable for other constructions like rollups, bridges,
beacon chains and execution layers, or in our case, a hierarchical architecture of Nakamoto-based consensus networks.

I would highly recommend everyone to read this paper if you are interested in the topic, but I will try to summarise the
core primitives that I found more interesting and that I am planning to leverage (either for inspiration or to evaluate
the correctness of our designs):

- *A formalised security framework for sidechains*
  : This is whithout a doubt one of the most relevant contributions of the paper. They formalise the concept of
  sidechains, present formal concepts for many of the primitives that are used in the construction of sidechains, and
  present a security and adversarial models that can be used to reason about the security of the system.
  - *Merged staking v.s. independent staking*: The paper presents two different models for the staking mechanism in
  sidechains. In the merged staking model, sidechains can leverage part of the staking power from the main chain to
  secure the network, i.e., the security of the sidechain is directly tied to the security of the main chain as some
  validators in the main chain also participate in the sidechain consensus. In the independent staking model, sidechains
  have their own staking mechanism, and their security is independent of the security of the main chain. The benefit of
  the merged staking model is that it helps preventing "goldfinger attacks" against sidechains with a small staking
  power.
  - *Direct observation v.s. certified-based observation*: It also introduces two different models for the relationship
  between sidechain and the flow of verifiable information between them. In the direct observation model, the sidechain
  directly observes the main chain, i.e. it participates as a full node of the sidechain and stores and verifies every
  state update in that network; while in the certified-based observation model, participants of other chains do not
  store all the state of a sidechain, and they rely on succint proofs (i.e. certificates) to verify the state of the a
  sidechain. In the paper, they present what they call an ad-hoc threshold multisignatures (ATMS) construction to enable
  the certified-based observation model.
  - *Firewall property*: The paper presents a formal definition of the firewall requirement that a sidechain must
  satisfy to ensure that the security of the main chain is not compromised by the sidechain. This property ensures that
  if a sidechain is compromised, the impact the attack can have over the main chain or other sidechains in the system is
  limited by the inflow of assets. The firewall property allows relying on an arbitrary definition of exactly how assets
  can correctly be moved back and forth between the two chains, we capture this by a so-called validity language. In
  case of failure, the firewall ensures that transfers from the sidechain into the main chain are rejected unless there
  exists a (not necessarily unique) plausible history of events on the sidechain that could, in case the sidechain
  was still secure, cause the particular transfers to take place.
  - *Merge operator*: It also presents abstractly the use of a merge operator that allow to sequentialise a set of
  transactions of two independent chains. `merge` allows us to create a combined view of multiple ledgers, putting all
  of the transactions across multiple ledgers into a linear ordering.
  - Finally, they show how their sidechain construction framework (i) supports safe cross-chain value transfers when the
  security assumptions of both chains are satisfied, namely that a majority of honest stake exists in both chains, and (
  ii) in case of a one-sided failure, maintains the firewall property, thus containing the damage to the chains whose
  security conditions have been violated.

## What does all of this involve for design of our system?

Inspired by the concepts from above, and after doing another thorough review
into [Subspace's consensus protocol](https://subspace.github.io/protocol-specs/docs/consensus/consensus_chain) (once
again, thank you Nazar for clarifying the rationale behind the design of some of the parts of the protocol) I started to
think about the different primitives that our design should have. These are still high-level ideas, but I am already
working to flesh out the design.

- If you recall from my [previous update](../2025-03-16-architecture-to-scale), our system will use an architecture
  composed by different layers of independent but interconnected chains (or shards), with the first layer being the
  global consensus (or main chain) responsible for orchestrating all the lower layers.
- We will adopt for our design an approach similar to the one presented for merged staking. All the farmers (we may also
  use consensus participants to refer to them more abstractly) dedicate their power to secure the global consensus, and
  they are also randomly assigned to participant and secure the consensus of the lower layers (by proposing and
  validating lower layer blocks). How I am planning to design this is that when a farmer audits its space to see if any
  of their chunks has a winning ticket to propose a block in the global consensus, they also check if they have a
  winning ticket to propose a block in any of the lower layers.
- To synchronise the different layers, I am planning to use a similar concept to that of epochs and slots. An epoch is a
  fixed period of time that is divided into slots. Each slot is assigned to a farmer that is responsible for proposing a
  block in the global consensus, and also for proposing a block in the lower layers. The global consensus will be
  responsible for orchestrating the assignment of slots to farmers in the lower layers, and for ensuring that the blocks
  proposed by the farmers in the lower layers are correctly included in the global consensus. Subspace already
  introduces an analogous approach through its proof-of-time that we may be able to leverage.
- We want Proof-of-Archival-Storage (PoAS) to apply to all the layers in the architecture, so that we can also use
  Proof-of-Space (PoSpace) for Sybil resistance. This is a key property that we want to maintain in our design, and one
  of the key things to figure out is how the protocol should work so that the history of all shards is archived and
  efficiently load balanced throughout the network to ensure their permanence and availability.
- In terms of chain observability, I expect that the lower layers will operate in a direct observation model, where they
  directly observe the global consensus, and they store and verify every state update in that network. We have an
  advantage here though, by using PoAS we make a the history of the shards available to all farmers, so that they can
  easily verify the state of the lower layers off-band if needed.
- Which brings me to the firewall property. In the paper when a sidechain fails, it is not recovering after that. The
  firewall property limits the impact of the attack, but there is no mechanism to make the sidechain operational again.
  In our case, the fact that we are archiving the history of all shards, and that we rely on a probabilistic consensus
  for their operation, will allow us to introduce a mechanism to recover from a failure of a shard. Thus, we want adopt
  the firewall property as described in the paper for our design and will try to come up with a stricter recovery
  property and mechanism.
- On top of all of this (and this is not something that I am currently considering as a high-priority) I am thinking
  about introducing the concept of a merge operator that allow to easily interleave the histories (or a subset of the
  history) between different shards. This operator will potentially be tightly coupled with
  the [execution model](/book/Execution_environment/Contracts_overview.html) that Nazar is working on, but I think that
  the modular approach that he is following can really help us to come up with a design that is flexible and that can be
  easily adapted to the different needs of the applications that will run on top of our system.
- Finally, the paper introduces the concepts of 2-way-pegs, cross-chain transactions, and the ability to perform atomic
  exchange of assets between chains. In our case, we are planning to limit the cross-shard operations to basic atomic
  primitives that allow to burn and mint tokens. This basic operation can be combined to construct more complex
  cross-chain operations by application developers. These operations will also be tightly coupled with the smart
  contract execution model (and already have many ideas on how to implement them), but I expect this to allow complex
  operation like the atomic execution of transactions involving states from two shards. In the scope of this, I had a
  few interesting discussions with Nazar about how addressing will work in our system, and how transactions would be
  queued for validations and routed throughout the hierarchy.
- As a side-note, I am leaving block rewards and the incentive model out of scope for this discussion for now, but I
  just want to let you know that we've also started thinking a little bit about this.

## What's next?

First of all let me thank you for reading this far. This update ended up becoming a little bit longer than usual, but I
wanted to share all the disconnected notes that I've been taking throughout the week. With all of these high-level ideas
in mind, this week I am planning to focus on designing in detail the proposal of blocks in the main chain and lower
level shards, and the archival of storage of the history of the shards. I think that with this we can start thinking
about the implementation of the core protocol so we can surface issues or other things that need further design, while
we work in parallel in detailing the rest of the system mechanics. Until next week!
