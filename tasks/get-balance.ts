import { Env, task } from "@terra-money/terrain";
import Lib from "../lib";

task(async (env:Env) => {
  const lib = new Lib(env);
  console.log("Getting balance...");
  let response = await lib.getBalance();
  console.log(response);
});