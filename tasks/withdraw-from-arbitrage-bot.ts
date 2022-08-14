import { Env, task } from "@terra-money/terrain";
import Lib from "../lib";

task(async (env:Env) => {
  const lib = new Lib(env);
  console.log("Withdrawing uluna from arbitrage bot...");
  let response = await lib.withdrawFromArbitrageBot("uluna");
  console.log(response);
});