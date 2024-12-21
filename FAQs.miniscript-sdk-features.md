# Q. What should a new SDK of Miniscript in a Language Contain as features ?

## Bitcoin Miniscript implementation in a new Language should contain these features

- miniscript parser
- type checker
- malleability check
- script length limit check (P2WSH standardness rule)
- op count check (P2WSH consensus rule)
- duplicate pubkey check
- script generation
- witness generation (satisfactions)
- unit tests that create receive addresses based on some miniscripts and spend from them again (see TestRedeem()).
- Unit tests exist to check that this implementation's type checking passes some of the unit tests of rust-miniscript.
- timelock mixing check
- stack size check (P2WSH standardness rule)
- exhaustive unit tests
- witness generation for thresh/multi currently has a naive implementation that is very inefficient (exponential runtime). This should be replace with a fast algorithm.

Ref: https://github.com/benma/miniscript-go

