import { Env, task } from "@terra-money/terrain";
import Lib from "../lib";
import * as fs from 'fs';

const arbitrageResultsFilename = "arbitrage-results.json";

task(async (env:Env) => {
  const lib = new Lib(env);
  while(true) {
    console.log("Estimating arbitrage...");
    let response = await lib.estimateArbitrage();
    let bestArbitrageOpportunity;
    if(Array.isArray(response)) {
      response.forEach((element) => {
        if(!bestArbitrageOpportunity || bestArbitrageOpportunity.simulated_profit < element.simulated_profit) {
          bestArbitrageOpportunity = element;
        }
      })
    }
    if (bestArbitrageOpportunity) {
      console.log("Executing arbitrage...");
      let executeArbitrage = await lib.executeArbitrage();
      const arbitrageResponse = {
        transactionHash: executeArbitrage.txhash,
        timestamp: executeArbitrage.timestamp,
        gas_used: executeArbitrage.gas_used,
      }
      //Check if file exists
      var arbitrage_results;
      if(fs.existsSync(arbitrageResultsFilename)) {
        var arbitrage_results_json = fs.readFileSync(arbitrageResultsFilename, "utf8");
        var arbitrage_results = JSON.parse(arbitrage_results_json);
        arbitrage_results.push(arbitrageResponse);
      } else {
        arbitrage_results = [];
        arbitrage_results.push(arbitrageResponse);
      }
      var jsonArbitrageResponse = JSON.stringify(arbitrage_results);
      fs.writeFile(arbitrageResultsFilename, jsonArbitrageResponse, 'utf8', function(err) {
        if (err) throw err;
        console.log("Executed arbitrage for profit: ", bestArbitrageOpportunity.simulated_profit);
      });
    } else {
      console.log("No arbitrage opportunity found!");
      await new Promise(resolve => setTimeout(resolve, 5000));
    }
  }
});