# Terra Flash Loan Arbitrage

Diplomska naloga - Veriga blokov Terra, arbitraža in Flash Loan. Diplomska naloga je na voljo za branje [tukaj](https://repozitorij.uni-lj.si/IzpisGradiva.php?id=140243&lang=slv).

## Jeziki

Na voljo v naslednjih jezikih:
- [Angleščina](README.md)
- [Slovenščina](README.si.md)

## Navodila za uporabo

1. V datoteki `contracts/arbitrage/src/exchanges/tokens` konstanti `TOKEN_LIST` dodajte poljubne žetone, preko katerih se bo izvajala arbitraža med menjalnicama `Astroport` in `Terraswap`. Poskrbi, da obstajajo Liquidity Pooli za LUNA-TOKEN na obeh menjalnicah, sicer bo prišlo do napake.
2. V korenski imenik projekta dodaj datoteko `.wallets.json` s sledečo strukturo (število denarnic ni omejeno):

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

3. Pripravljeni smo na namestitev pametnih pogodb.

    3.1. Obvezno je pričeti z namestitvijo pametne pogodbe `flash-loan`, ker to uporablja pametna pogodba `arbitrage`.
    
    3.2. Pametno pogodbo namestimo z naslednjim ukazom:

    ```
    terrain deploy flash-loan --signer terra_wallet_1 --set-signer-as-admin --network testnet
    ```

    `--signer` označuje denarnico, ki bo namestila pametno pogodbo. Privzeto je to prva denarnica v datoteki `.wallets.json`. Za spreminjanje uredi datoteko `keys.terrain.json`.

    `--set-singer-as-admin` omogoča tej denarnici migriranje pametne pogodbe. To pomeni, da ni potrebna ponovna namestitev, ampak se naslov in pretekle transakcije shranijo.

    `--network` pove na katero omrežje nameščamo pametno pogodbo.

    3.3. V datoteki `config.terrain.json` polje `flash_loan_contract_address` nastavimo na naslov kamor je nameščena pametna pogodba za flash loan.

    3.4. Nadaljujemo z namestitvijo pametne pogodbe `arbitrage`:

    ```
    terrain deploy arbitrage --signer terra_wallet_1 --set-signer-as-admin --network testnet
    ```

    3.* Za migracijo (ponovna namestitev, ki ohrani transkacije in naslov) uporabimo naslednji ukaz:

    ```
    terrain contract:migrate <ime_pametne_pogodbe> --signer <admin_denarnica> --network testnet
    ```

4. V imeniku `tasks` imamo podprte vse metode, ki jih podpirajo pametne pogobe.

5. Posodobimo nastavitve pametne pogodbe `flash-loan`, da lahko z njo upravljamo iz pametne pogodbe `arbitrage`.

```
terrain task:run update-config --network testnet
```

![Updating the flash loan smart contract config!](/arbitrage_pictures/flash_loan_config_update.png "Updated Flash Loan Config")

6. Na pametno pogodbo `arbitrage` pošljemo žeton `LUNA`. V datoteki `tasks/provide-to-flash-loan` spremenimo vrednost `100000000` v vrednost, ki smo jo nakazali na pametno pogodbo `arbitrage`, * 10<sup>6</sup> (mikro luna). Te žetone nato posredujemo na pametno pogodbo `flash-loan` z ukazom:

```
terrain task:run provide-to-flash-loan --network testnet
```

![Providing funds to the flash loan!](/arbitrage_pictures/provided_funds_to_flash_loan.png "Provided funds to Flash Loan")

7. Z nalogo `get-balance` potrdimo, da so sredstva uspešno nakazana:

```
terrain task:run get-balance --network testnet
```

![Checking the balance on the flash loan!](/arbitrage_pictures/flash_loan_balance.png "Flash Loan Balance")

8. Zapomnimo se trenutne vrednost žetonov na pametni pogodbi `arbitrage`:

```
terrain task:run query-balances --network testnet
```

![Checking current balances on arbitrage smart contract!](/arbitrage_pictures/arbitrage_smart_contract_balance_before.png "Balance before the arbitrage")

9. Ocenimo arbitražo z ukazom:

```
terrain task:run estimate-arbitrage --network testnet
```

![Estimate the arbitrage!](/arbitrage_pictures/arbitrage_estimation.png "Arbitrage estimation")

10. V kolikor je prisotna priložnost za arbitražo, to izvedemo z nalogo `execute-arbitrage`:

```
terrain task:run execute-arbitrage --network testnet
```

![Execute the arbitrage!](/arbitrage_pictures/executing_arbitrage.png "Arbitrage execution")

11. Zopet preverimo vrednosti žetonov na pametni pogodbi `arbitrage`, da vidimo naš zaslužek:

```
terrain task:run query-balances --network testnet
```

![Checking balances after arbitrage!](/arbitrage_pictures/arbitrage_smart_contract_balance_after.png "Balances after arbitrage")

12. Po želji prenesemo žetone iz pametne pogodbe `arbitrage` na administratorsko denarnico (denarnica, ki je poskrbela za namestitev v omrežje):

```
terrain task:run withdraw-from-arbitrage-bot --network testnet
```