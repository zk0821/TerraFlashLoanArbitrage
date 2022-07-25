import { Env } from "@terra-money/terrain";

export class Lib {
  env: Env;

  constructor(env: Env){
    this.env = env;
  }

  getCount = (env = this.env) => {
    return env.client.query("counter", { get_count: {} })
  }

  queryPool = (address: string, env = this.env) => {
    return env.client.query("arb", { query_pool: { address } })
  }

  simulateNativeTokenSwap = (address: string, amount: string, denom: string, env = this.env) => {
    return env.client.query("arb", { simulate_native_token_swap: { address, amount, denom }})
  }

  simulateTokenSwap = (address: string, amount: string, contract_addr: string, env = this.env) => {
    return env.client.query("arb", { simulate_token_swap: { address, amount, contract_addr }})
  }

  estimateArbitrage = (env = this.env) => {
    return env.client.query("arb", { estimate_arbitrage: { }})
  }

};

export default Lib;