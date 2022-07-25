import { Env, task } from "@terra-money/terrain";
import Lib from '../lib';

task(async (env: Env) => {
  const lib = new Lib(env);
  //Astroport pool
  await lib.queryPool("terra1udsua9w6jljwxwgwsegvt6v657rg3ayfvemupnes7lrggd28s0wq7g8azm");
  //Terraswap pool
  await lib.queryPool("terra1ksu84lkky4pshnu2dyqvfvk789ypvlykhtqrk9nsjfsh9t5qy9dsaf8r04");
  //Astroport swap
  await lib.simulateNativeTokenSwap("terra1udsua9w6jljwxwgwsegvt6v657rg3ayfvemupnes7lrggd28s0wq7g8azm", "100", "uluna");
  //Terraswap swap
   await lib.simulateTokenSwap("terra1ksu84lkky4pshnu2dyqvfvk789ypvlykhtqrk9nsjfsh9t5qy9dsaf8r04", "100", "terra167dsqkh2alurx997wmycw9ydkyu54gyswe3ygmrs4lwume3vmwks8ruqnv");
  //Estimate arbitrage
  await lib.estimateArbitrage()
});
