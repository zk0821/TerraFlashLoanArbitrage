import { Env, task } from "@terra-money/terrain";
import Lib from "../lib";

task(async (env:Env) => {
  const lib = new Lib(env);
  console.log("Providing to flash loan smart contract...");
  let response = await lib.provideToFlashLoan("uluna", "100000000");
  console.log(response);
  let balance = await lib.getBalance();
  console.log("Balance: ", balance);
});