import { Env } from "@terra-money/terrain";
import { BlockTxBroadcastResult } from "@terra-money/terra.js";

export class Lib {
  env: Env;

  constructor(env: Env){
    this.env = env;
  }

  /* ARBITRAGE METHODS */
  queryBalances = (env = this.env) => {
    return env.client.query("arbitrage", { query_balances: { } })
  }

  queryPool = (factory_addr: string, offer_asset: string, wanted_asset:string, env = this.env) => {
    return env.client.query("arbitrage", { query_pool: { factory_addr, offer_asset, wanted_asset } })
  }

  simulateSwap = (factory_addr: String, offer_asset: String, amount: String, wanted_asset: String, env = this.env) => {
    return env.client.query("arbitrage", { simulate_swap: { factory_addr, offer_asset, amount, wanted_asset } })
  }

  estimateArbitrage = (env = this.env) => {
    return env.client.query("arbitrage", { estimate_arbitrage: { }})
  }

  executeSwap = (factory_addr: string, offer_asset: string, offer_amount: string, wanted_asset: string, env = this.env): Promise<BlockTxBroadcastResult> => {
    return env.client.execute(env.wallets.terra_wallet_1, "arbitrage", { execute_swap: { factory_addr, offer_asset, offer_amount, wanted_asset } })
  }

  provideToFlashLoan = (offer_asset: string, offer_amount: string, env = this.env): Promise<BlockTxBroadcastResult> => {
    return env.client.execute(env.wallets.terra_wallet_1, "arbitrage", { provide_to_flash_loan: { offer_asset, offer_amount } })
  }

  withdrawFromFlashLoan = (env = this.env): Promise<BlockTxBroadcastResult> => {
    return env.client.execute(env.wallets.terra_wallet_1, "arbitrage", { withdraw_from_flash_loan: { } })
  }

  withdrawFromArbitrageBot = (denom: string, env = this.env): Promise<BlockTxBroadcastResult> => {
    return env.client.execute(env.wallets.terra_wallet_1, "arbitrage", { withdraw: { denom } })
  }

  executeArbitrage = (env = this.env): Promise<BlockTxBroadcastResult> => {
    return env.client.execute(env.wallets.terra_wallet_1, "arbitrage", { execute_arbitrage: {  } })
  }

  /* FLASH LOAN METHODS */
  getConfig = (env = this.env) => {
    return env.client.query("flash-loan", { get_config: { } })
  }

  getBalance = (env = this.env) => {
    return env.client.query("flash-loan", { get_balance: { } })
  }

  updateConfig = (owner: string, fee: string, env = this.env): Promise<BlockTxBroadcastResult> => {
    return env.client.execute(env.wallets.terra_wallet_1, "flash-loan", { update_config: { owner, fee } })
  }
};

export default Lib;