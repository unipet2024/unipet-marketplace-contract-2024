import * as anchor from "@coral-xyz/anchor";

import { PublicKey } from "@solana/web3.js";

import { Wallet } from "@coral-xyz/anchor";

import { program, provider, connection } from "./helper";

import {
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
let owner = provider.wallet as Wallet;

const payer = owner.payer;

async function buy_spl() {
  const buyer = new PublicKey("aGwtDcFXg9FMJ43axF1x1wqeVjPSLHeVGhmgEGgWn16");
  const seller = new PublicKey("aGwtDcFXg9FMJ43axF1x1wqeVjPSLHeVGhmgEGgWn16");
  const MINT = new PublicKey("5Et3fqFdXqKRKnTvNq8YBrdYWfQdSALJFYiCsjKdHAL7");
  const USDC = new PublicKey("BUJST4dk6fnM5G3FnhTVc3pjxRJE7w2C5YL9XgLbdsXW");
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  let [market_account] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("MARKET_ACCOUNT")],
    program.programId
  );

  console.log("Market: ", market_account.toString());

  const mint_market = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    MINT,
    market_account,
    true
  );

  console.log("MINT Market: ", mint_market.address.toString());

  const usdc_market = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    USDC,
    market_account,
    true
  );

  console.log("USDC market: ", usdc_market.address.toString());

  const mint_buyer = await getAssociatedTokenAddress(MINT, buyer);
  console.log("MINT buyer: ", mint_buyer.toString());

  const mint_buyer_balance = await connection.getTokenAccountBalance(
    mint_buyer
  );
  console.log(
    "MINT buyer balance: ",
    mint_buyer_balance.value.amount.toString()
  );

  const usdc_buyer = await getAssociatedTokenAddress(USDC, buyer);
  console.log("USDC buyer: ", usdc_buyer.toString());

  const usdc_buyer_balance = await connection.getTokenAccountBalance(
    usdc_buyer
  );
  console.log(
    "USDC buyer balance: ",
    usdc_buyer_balance.value.amount.toString()
  );

  const usdc_seller = await getAssociatedTokenAddress(USDC, seller);
  console.log("USDC seller: ", usdc_buyer.toString());

  let [listing_account] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("LISTING_ACCOUNT"), MINT.toBuffer()],
    program.programId
  );

  console.log("Listing account: ", listing_account.toString());

  const listing_account_info = await program.account.listingData.fetch(
    listing_account
  );

  console.log(listing_account_info);

  try {
    await program.methods
      .buyWithSpl()
      .accounts({
        market: market_account,
        nftTo: mint_buyer,
        currencyFrom: usdc_buyer,
        currencyMarket: usdc_market.address,
        currencyTo: usdc_seller,
        nftFrom: mint_market.address,
        nftMint: MINT,
        listingAccount: listing_account,
        seller: seller,
        // buyer: buyer,
        currencyMint: USDC,
      })
      .rpc();
  } catch (error) {
    console.log(error);
  }

  let market_account_info = await program.account.market.fetch(market_account);
  console.log(market_account_info);

  // let market_account_info= await program.account.market.fetch(market_account)
}

buy_spl();
