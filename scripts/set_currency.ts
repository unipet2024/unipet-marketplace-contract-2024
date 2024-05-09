import * as anchor from "@coral-xyz/anchor";
import { Wallet } from "@coral-xyz/anchor";

import { PublicKey } from "@solana/web3.js";

import { program, provider } from "./helper";

async function init() {
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  let [market_account] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("MARKET_ACCOUNT")],
    program.programId
  );

  console.log("Market: ", market_account.toString());

  let [operator_account] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("OPERATOR_ROLE")],
    program.programId
  );

  console.log("Operator: ", operator_account.toString());

  console.log("Set currency");

  const unp_token = new PublicKey(
    "5Et3fqFdXqKRKnTvNq8YBrdYWfQdSALJFYiCsjKdHAL7"
  );
  const usdc_token = new PublicKey(
    "BUJST4dk6fnM5G3FnhTVc3pjxRJE7w2C5YL9XgLbdsXW"
  );

  const address_0 = new PublicKey("11111111111111111111111111111111");

  try {
    await program.methods
      .setCurrencies({ currency: [unp_token, usdc_token, address_0] })
      .accounts({
        market: market_account,
        operatorAccount: operator_account,
      })
      .rpc();
  } catch (error) {
    console.log(error);
  }

  let market_account_info = await program.account.market.fetch(market_account);
  console.log(market_account_info);
}

init();
