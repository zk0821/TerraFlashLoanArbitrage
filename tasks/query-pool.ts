import { Env, task } from "@terra-money/terrain";
import Lib from "../lib";
import { TERRASWAP_FACTORY_ADDR, ASTROPORT_FACTORY_ADDR, ULUNA, ASTRO} from "./contract-addresses";

task(async (env:Env) => {
  const lib = new Lib(env);
  console.log("Testing terraswap pool query...");
  let responseTerraswap = await lib.queryPool(TERRASWAP_FACTORY_ADDR, ULUNA, ASTRO);
  console.log(responseTerraswap);
  console.log("Testing astroport pool query...");
  let responseAstroport = await lib.queryPool(ASTROPORT_FACTORY_ADDR, ULUNA, ASTRO);
  console.log(responseAstroport);
});