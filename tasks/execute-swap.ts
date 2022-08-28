import { Env, task } from "@terra-money/terrain";
import Lib from "../lib";
import { TERRASWAP_FACTORY_ADDR, ASTROPORT_FACTORY_ADDR, ULUNA, LIRA } from "./contract-addresses";

task(async (env:Env) => {
  const lib = new Lib(env);
  console.log("Testing terraswap swap...");
  let terraswapResponse = await lib.executeSwap(TERRASWAP_FACTORY_ADDR, ULUNA, "1", LIRA);
  console.log(terraswapResponse);
  console.log("Testing astroport swap...");
  let astroportResponse = await lib.executeSwap(ASTROPORT_FACTORY_ADDR, ULUNA, "1", LIRA);
  console.log(astroportResponse);
});