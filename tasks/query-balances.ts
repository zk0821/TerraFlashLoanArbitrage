import { Env, task } from "@terra-money/terrain";
import Lib from "../lib";

task(async (env:Env) => {
  const lib = new Lib(env);
  console.log("Querying balances...");
  let response = await lib.queryBalances();
  console.log(response);
});