To create this MVP I butchered liquidity pool example from [soroban-examples](https://github.com/stellar/soroban-examples/tree/main), hence the names.

When token is created, it's very easy to reach some call limit.

To reproduce the issue, go into `liquidity_pool` directory and call `make test`.

```
running 1 test
test test::test ... FAILED

failures:

---- test::test stdout ----
thread 'test::test' panicked at 'HostError: Error(Budget, ExceededLimit)

Event log (newest first):
   0: [Diagnostic Event] topics:[error, Error(Budget, ExceededLimit)], data:"escalating error to panic"
   1: [Diagnostic Event] topics:[error, Error(Budget, ExceededLimit)], data:["contract call failed", deposit, [Address(Contract(1b7dcd0c27d085dc01927c249e11a6a226bdb8d9fe43a655e4e4f5fe28996829)), Bytes(85fd39ea929a326a974a3ffcec0626508d1dd81965c9cde9707f370647d292bc), Address(Contract(399ab3a6e21aeec0c5cca79660946fbb840ea8b5b9431c3bd45c71ad920301ce)), Address(Contract(2ef1741672b934d70b2f0f0bf1f3bd831244d1a6dedcf9c4bd620ea0b2de8967))]]
```

```
...
   6: soroban_liquidity_pool_contract::LiquidityPoolClient::deposit
             at src/lib.rs:28:1
   7: soroban_liquidity_pool_contract::test::test
             at src/test.rs:31:5
   8: soroban_liquidity_pool_contract::test::test::{{closure}}
             at src/test.rs:20:11
   9: core::ops::function::FnOnce::call_once
             at /rustc/8ede3aae28fe6e4d52b38157d7bfe0d3bceef225/library/core/src/ops/function.rs:250:5
```
