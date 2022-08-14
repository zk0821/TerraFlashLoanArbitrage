import { Env, task } from "@terra-money/terrain";
import Lib from "../lib";

task(async (env:Env) => {
  const lib = new Lib(env);
  console.log("Getting config...");
  let response = await lib.getConfig();
  console.log(response);
  console.log("Updating config...")
  await lib.updateConfig(env.refs.arbitrage.contractAddresses.default, "0.009")
  console.log("Getting config...");
  let responseAfterUpdate = await lib.getConfig();
  console.log(responseAfterUpdate);
});