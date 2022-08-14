import { Env, task } from "@terra-money/terrain";
import Lib from "../lib";

task(async (env:Env) => {
  const lib = new Lib(env);
  console.log("Estimating arbitrage...");
  let response = await lib.estimateArbitrage();
  console.log(response);
});