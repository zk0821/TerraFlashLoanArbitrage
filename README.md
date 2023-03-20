# Terra Flash Loan Arbitrage

This is a product of my Bachelor's Thesis. The whole paper is available [here](https://repozitorij.uni-lj.si/IzpisGradiva.php?id=140243&lang=slv).

Note: The paper is written in Slovene.

## Languages

Read this in other languages:

- [English](README.md)
- [Slovene](README.si.md)

## Setup

This is the setup I used to find and execute arbitrage opportunities.

### Tokens

In file `contracts/arbitrage/src/exchanges/tokens` change the constant `TOKEN_LIST` to include tokens you wish to find and execute arbitrage opportunities for. These tokens should be present on both `Astroport` and `Terraswap`, the main two exchanges on Terra.

Note: Tokens are currently only matched against LUNA. This means that in order for the arbitrage bot to work there should exist a Liqudity Pool LUNA-TOKEN (for each token in `TOKEN_LIST`) on both exchanges!

### Wallets

In the root directory add file `.wallets.json` with the following structure:

```json
    {
        "wallets": [
            {
                "wallet_name": "terra-wallet-1",
                "mnemonic": "outer genuine lake cactus valid video erode sound birth tenant athlete good sweet salute panic point frost grass vintage leave peasant found slice mixture",
            }
        ]
    }
```

Note: Number of wallets is not limited.

### Smart Contract Deployment

1. It is necessary to begin with deployment of `flash-loan` smart contract, because `arbitrage` smart contract uses it.

2. Deploy the smart contract with the following command:

    ```
    terrain deploy flash-loan --signer terra_wallet_1 --set-signer-as-admin --network testnet
    ```

    `--signer` denotes the wallet, which will deploy the smart contract. By default this is the first wallet in file `.wallets.json`. To change this behaviour modify `keys.terrain.json`.

    `--set-signer-as-admin` enables this wallet to migrate the smart contract. This means that when deploying updates all past transaction along with the address of the smart contract are saved.

    `--network` specifies to which network we are deploying the smart contract.

3. In file `config.terrain.json` change the field `flash_loan_contract_address` to the address of the previously deployed flash loan smart contract.

4. Deploy the `arbitrage` smart contract:

    ```
    terrain deploy arbitrage --signer terra_wallet_1 --set-signer-as-admin --network testnet
    ```

Note: To migrate the smart contract(re-deploy that keeps transaction history and address) use the following command:

```
terrain contract:migrate <ime_pametne_pogodbe> --signer <admin_denarnica> --network testnet
```

### Tasks

Directory `tasks` includes tasks for all methods the smart contracts support.

#### Update flash-loan smart contract configuration

This will allow us to execute methods from `flash-loan` smart contracts directly from `arbitrage` smart contract.

Run the following command:

```
terrain task:run update-config --network testnet
```

Result:
![Updating the flash loan smart contract config!](/arbitrage_pictures/flash_loan_config_update.png "Updated Flash Loan Config")

#### Provide tokens to flash-loan smart contract

1. Send `LUNA` tokens to `arbitrage` smart contract.
2. In file `tasks/provide-to-flash-loan.ts` change value `100000000` to the amount we transferred to the smart contract. (Note: this is measured in ULUNA = LUNA * 10<sup>6</sup>)
3. Transfer these tokens to `flash-loan` smart contract using the following command:

    ```
    terrain task:run provide-to-flash-loan --network testnet
    ```

    Result:
    ![Providing funds to the flash loan!](/arbitrage_pictures/provided_funds_to_flash_loan.png "Provided funds to Flash Loan")

#### Check if balance on flash-loan smart contract is correct

Confirm the tokens have been successfully sent to the `flash-loan` smart contract by running:

```
terrain task:run get-balance --network testnet
```

Result:
![Checking the balance on the flash loan!](/arbitrage_pictures/flash_loan_balance.png "Flash Loan Balance")

#### Check current tokens status on arbitrage smart contract

```
terrain task:run query-balances --network testnet
```

Result:
![Checking current balances on arbitrage smart contract!](/arbitrage_pictures/arbitrage_smart_contract_balance_before.png "Balance before the arbitrage")

#### Estimate the arbitrage

We can estimate the arbitrage by running:

```
terrain task:run estimate-arbitrage --network testnet
```

Result:
![Estimate the arbitrage!](/arbitrage_pictures/arbitrage_estimation.png "Arbitrage estimation")

#### Execute the arbitrage

If we discover that an opportunity for arbitrage exists we can execute the arbitrage and make some profit.

```
terrain task:run execute-arbitrage --network testnet
```

Result:
![Execute the arbitrage!](/arbitrage_pictures/executing_arbitrage.png "Arbitrage execution")

#### Check balance after arbitrage

Check how much profit was made from that last arbitrage by comparing current balance to the one before the arbitrage.

```
terrain task:run query-balances --network testnet
```

Result:
![Checking balances after arbitrage!](/arbitrage_pictures/arbitrage_smart_contract_balance_after.png "Balances after arbitrage")

#### Transfer profit from arbitrage smart contract to personal wallet

Transfer the profit made by the executed arbitrage into our own personal wallet by using the command:

```
terrain task:run withdraw-from-arbitrage-bot --network testnet
```