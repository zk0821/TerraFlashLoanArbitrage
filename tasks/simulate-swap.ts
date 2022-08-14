import { Env, task } from "@terra-money/terrain";
import Lib from "../lib";
import { TERRASWAP_FACTORY_ADDR, ASTROPORT_FACTORY_ADDR, ULUNA, ASTRO} from "./contract-addresses";

task(async (env:Env) => {
  const lib = new Lib(env);
  console.log("Simulate Terraswap 100 uluna -> ASTRO");
  let responseTerraswap = await lib.simulateSwap(TERRASWAP_FACTORY_ADDR, ULUNA, "100", ASTRO);
  console.log(responseTerraswap);
  console.log("Simulate Astroport 100 uluna -> ASTRO");
  let responseAstroport = await lib.simulateSwap(ASTROPORT_FACTORY_ADDR, ULUNA, "100", ASTRO);
  console.log(responseAstroport);
});