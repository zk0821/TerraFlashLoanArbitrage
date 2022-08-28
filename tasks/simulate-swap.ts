import { Env, task } from "@terra-money/terrain";
import Lib from "../lib";
import { TERRASWAP_FACTORY_ADDR, ASTROPORT_FACTORY_ADDR, ULUNA, STEAK, LUNAX, LIRA, ALEM, AMPLUNA} from "./contract-addresses";

task(async (env:Env) => {
  const lib = new Lib(env);
  console.log("Simulate Terraswap 100 uluna -> ASTRO");
  let responseTerraswap = await lib.simulateSwap(TERRASWAP_FACTORY_ADDR, ULUNA, "10000000", STEAK);
  console.log(responseTerraswap);
  console.log("Simulate Astroport 100 uluna -> ASTRO");
  let responseAstroport = await lib.simulateSwap(ASTROPORT_FACTORY_ADDR, STEAK, String(responseTerraswap), ULUNA);
  console.log(responseAstroport);
  console.log("REVERSE")
  console.log("Simulate Terraswap 100 uluna -> ASTRO");
  let responseTerraswap1 = await lib.simulateSwap(ASTROPORT_FACTORY_ADDR, ULUNA, "10000000", STEAK);
  console.log(responseTerraswap1);
  console.log("Simulate Astroport 100 uluna -> ASTRO");
  let responseAstroport2 = await lib.simulateSwap(TERRASWAP_FACTORY_ADDR, STEAK, String(responseTerraswap1), ULUNA);
  console.log(responseAstroport2);
  //
  console.log("Simulate Terraswap 100 uluna -> ASTRO");
  responseTerraswap = await lib.simulateSwap(TERRASWAP_FACTORY_ADDR, ULUNA, "10000000", LUNAX);
  console.log(responseTerraswap);
  console.log("Simulate Astroport 100 uluna -> ASTRO");
  responseAstroport = await lib.simulateSwap(ASTROPORT_FACTORY_ADDR, LIRA, String(responseTerraswap), ULUNA);
  console.log(responseAstroport);
  //
  console.log("Simulate Terraswap 100 uluna -> ASTRO");
  responseTerraswap = await lib.simulateSwap(TERRASWAP_FACTORY_ADDR, ULUNA, "10000000", ALEM);
  console.log(responseTerraswap);
  console.log("Simulate Astroport 100 uluna -> ASTRO");
  responseAstroport = await lib.simulateSwap(ASTROPORT_FACTORY_ADDR, AMPLUNA, String(responseTerraswap), ULUNA);
  console.log(responseAstroport);
});