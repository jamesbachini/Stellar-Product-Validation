# Stellar On-Chain Data Landing Page Tutorial

This repository demonstrates how to use web3 technology to store data on-chain rather than in a centralized database when building a landing page. In this tutorial, you'll learn how to deploy a simple Soroban smart contract that maintains an individual counter for each wallet address and integrates with a basic web landing page.

---

## Overview

In this project, we use a modified version of the Soroban increment contract. The smart contract stores a counter for each user and offers two public functions:

- **increment** – Increases the counter for the authenticated wallet address.
- **read** – Returns the current counter value for a given wallet address (this function is free to call as it is executed as a simulated transaction).

On the front end, a simple HTML landing page is provided to interact with the smart contract using the Stellar SDK, allowing users to trigger transactions that write to and read from the blockchain.

---

## Prerequisites

Before getting started, make sure you have the following installed:

- **Rust** – [Download Rust](https://www.rust-lang.org/)
- **Stellar CLI** – [Stellar CLI Documentation](https://developers.stellar.org/docs/build/guides/cli)
- **Testnet Funds** (free) – [Create a Testnet Account](https://lab.stellar.org/account/create)

---

**Note:** The `read` function is free to call since it simulates a transaction and does not modify state.

---

## Deploying the Smart Contract

You can deploy the contract to the Stellar Testnet by following these steps:

1. **Compile the Contract:**

   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

2. **Deploy the Contract:**

   You can deploy your contract using the Stellar CLI. Replace `james` with your source account identifier if necessary. Alternatively, you can use the provided testnet contract address below:

   ```
   CASEN2ZFCC5MY3QQSWPFPGZZIM3VM5GSTASKBKO7AO7UCFH4NJ2PGGOA
   ```

   **Deployment Command:**

   ```bash
   stellar contract deploy \
     --wasm target/wasm32-unknown-unknown/release/multi_user_increment.wasm \
     --source james \
     --network testnet
   ```

**Security Note:** Any user can interact with this contract, and all on-chain data is public. Ensure that you do not store any private or sensitive information unless it is properly encrypted.

---

## Landing Page

The landing page (`index.html`) serves as a simple interface to validate a product idea by interacting with the on-chain data.

### Importing the Stellar SDK

The landing page includes the Stellar SDK to handle the on-chain transactions:

```html
<script src="https://cdnjs.cloudflare.com/ajax/libs/stellar-sdk/13.1.0/stellar-sdk.js"></script>
```

### Writing Transactions

The following JavaScript snippet shows how to send a write transaction that calls the `increment` function:

```javascript
const rpc = new StellarSdk.rpc.Server('https://soroban-testnet.stellar.org');
const contract = new StellarSdk.Contract(CONTRACT_ID);
const networkPassphrase = StellarSdk.Networks.TESTNET;
const sourceKeypair = StellarSdk.Keypair.fromSecret(SECRET_KEY);
const scValAddress = StellarSdk.nativeToScVal(PUBLIC_KEY, { type: "address" });
const sourceAccount = await rpc.getAccount(PUBLIC_KEY);

const tx = new StellarSdk.TransactionBuilder(sourceAccount, {
    fee: StellarSdk.BASE_FEE,
    networkPassphrase: networkPassphrase,
})
.addOperation(contract.call("increment", scValAddress))
.setTimeout(30)
.build();

const preparedTx = await rpc.prepareTransaction(tx);
preparedTx.sign(sourceKeypair);
const txResult = await rpc.sendTransaction(preparedTx);
console.log('txResult', txResult);
```

### Reading Transactions

To read the counter for a wallet address using the `read` function, the following JavaScript code simulates a transaction:

```javascript
const tx = new StellarSdk.TransactionBuilder(sourceAccount, {
    fee: StellarSdk.BASE_FEE,
    networkPassphrase: networkPassphrase,
})
.addOperation(contract.call("read", scValAddress))
.setTimeout(30)
.build();

rpc.simulateTransaction(tx).then((sim) => {
    const decoded = StellarSdk.scValToNative(sim.result?.retval);
    alert(`Clicks: ${decoded}`);
});
```

---

## Full Code & Additional Resources

You can find the complete project code and additional instructions here:  
[Stellar Product Validation Repository](https://github.com/jamesbachini/Stellar-Product-Validation)

For more detailed guidance on Stellar development, check out these resources:

- **Stellar Dev Portal:** [Stellar Dev Hub](https://developers.stellar.org/)
- **Stellar CLI Documentation:** [Stellar CLI Guides](https://developers.stellar.org/docs/build/guides/cli)

---

## License

This project is licensed under the MIT License.

---

By following this tutorial, you'll be able to leverage web3 technology to store and interact with on-chain data, validating product ideas without relying on traditional databases. Enjoy exploring the decentralized world of Stellar and Soroban.