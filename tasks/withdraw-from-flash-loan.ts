import { Env, task } from "@terra-money/terrain";
import { loadConfig } from "@terra-money/terrain/lib/config";
import Lib from "../lib";

task(async (env:Env) => {
  const lib = new Lib(env);
  console.log("Withdrawing from the flash loan smart contract...");
  let responseWithdraw = await lib.withdrawFromFlashLoan();
  console.log(responseWithdraw);
  let balance = await lib.getBalance();
  console.log("Balance: ", balance);
});