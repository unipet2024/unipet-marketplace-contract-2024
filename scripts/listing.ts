import * as anchor from "@coral-xyz/anchor";
import { Wallet } from "@coral-xyz/anchor";

import { PublicKey } from "@solana/web3.js";

import { program, provider, connection } from "./helper";
import {
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";

let owner = provider.wallet as Wallet;
const payer = owner.payer;

async function init() {
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  // const MINT = new PublicKey("5Et3fqFdXqKRKnTvNq8YBrdYWfQdSALJFYiCsjKdHAL7");
  const operator = new PublicKey("aGwtDcFXg9FMJ43axF1x1wqeVjPSLHeVGhmgEGgWn16");
  const USDC = new PublicKey("BUJST4dk6fnM5G3FnhTVc3pjxRJE7w2C5YL9XgLbdsXW");

  const mint_from = await getAssociatedTokenAddress(USDC, operator);

  let [listing_mint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("LISTING_ACCOUNT"), USDC.toBuffer()],
    program.programId
  );

  console.log("Listing mint: ", listing_mint.toString());

  let [market_account] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("MARKET_ACCOUNT")],
    program.programId
  );

  let market_mint = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    USDC,
    market_account,
    true
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
    const transaction = await program.methods
      .listing(address_0, new anchor.BN(100))
      .accounts({
        market: market_account,
        operatorAccount: operator_account,
        listingAccount: listing_mint,
        from: mint_from,
        to: market_mint.address,
        mint: USDC,
      })
      .rpc();

    console.log(transaction);
  } catch (error) {
    console.log(error);
  }

  let market_account_info = await program.account.market.fetch(market_account);
  console.log(market_account_info);
}

init();
