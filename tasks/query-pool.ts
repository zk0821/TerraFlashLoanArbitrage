import { Env, task } from "@terra-money/terrain";
import Lib from "../lib";
import { TERRASWAP_FACTORY_ADDR, ASTROPORT_FACTORY_ADDR, ULUNA, STEAK, LUNAX, LIRA, ALEM, AMPLUNA} from "./contract-addresses";

task(async (env:Env) => {
  const lib = new Lib(env);
  console.log("Testing terraswap pool query...");
  let responseTerraswap = await lib.queryPool(TERRASWAP_FACTORY_ADDR, ULUNA, STEAK);
  console.log(responseTerraswap);
  console.log("Testing astroport pool query...");
  let responseAstroport = await lib.queryPool(ASTROPORT_FACTORY_ADDR, ULUNA, STEAK);
  console.log(responseAstroport);
  //
  console.log("Testing terraswap pool query...");
  responseTerraswap = await lib.queryPool(TERRASWAP_FACTORY_ADDR, ULUNA, LUNAX);
  console.log(responseTerraswap);
  console.log("Testing astroport pool query...");
  responseAstroport = await lib.queryPool(ASTROPORT_FACTORY_ADDR, ULUNA, LUNAX);
  console.log(responseAstroport);
  //
  console.log("Testing terraswap pool query...");
  responseTerraswap = await lib.queryPool(TERRASWAP_FACTORY_ADDR, ULUNA, LIRA);
  console.log(responseTerraswap);
  console.log("Testing astroport pool query...");
  responseAstroport = await lib.queryPool(ASTROPORT_FACTORY_ADDR, ULUNA, LIRA);
  console.log(responseAstroport);
  //
  console.log("Testing terraswap pool query...");
  responseTerraswap = await lib.queryPool(TERRASWAP_FACTORY_ADDR, ULUNA, ALEM);
  console.log(responseTerraswap);
  console.log("Testing astroport pool query...");
  responseAstroport = await lib.queryPool(ASTROPORT_FACTORY_ADDR, ULUNA, ALEM);
  console.log(responseAstroport);
  //
  console.log("Testing terraswap pool query...");
  responseTerraswap = await lib.queryPool(TERRASWAP_FACTORY_ADDR, ULUNA, AMPLUNA);
  console.log(responseTerraswap);
  console.log("Testing astroport pool query...");
  responseAstroport = await lib.queryPool(ASTROPORT_FACTORY_ADDR, ULUNA, AMPLUNA);
  console.log(responseAstroport);
});