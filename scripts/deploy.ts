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

  //   const unp_token = new PublicKey(
  //     "5Et3fqFdXqKRKnTvNq8YBrdYWfQdSALJFYiCsjKdHAL7"
  //   );
  const USDC = new PublicKey("BUJST4dk6fnM5G3FnhTVc3pjxRJE7w2C5YL9XgLbdsXW");

  const address_0 = new PublicKey("11111111111111111111111111111111");

  let [market_account] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("MARKET_ACCOUNT")],
    program.programId
  );

  console.log("Market: ", market_account.toString());

  let [admin_account] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("ADMIN_ROLE")],
    program.programId
  );

  console.log("Admin: ", admin_account.toString());

  let [operator_account] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("OPERATOR_ROLE")],
    program.programId
  );

  console.log("Operator: ", operator_account.toString());

  console.log("-------------INIT-------------");

  try {
    await program.methods
      .init(
        new anchor.BN(300),
        { currency: [USDC, address_0] },
        new anchor.BN(20)
      )
      .accounts({
        market: market_account,
        adminAccount: admin_account,
        operatorAccount: operator_account,
      })
      .rpc();
  } catch (error) {
    console.log(error);
  }

  try {
    console.log(
      "----------------------Set market to public-------------------"
    );
    await program.methods
      .setStatus({ public: {} })
      .accounts({
        market: market_account,
        operatorAccount: operator_account,
        adminAccount: admin_account,
      })
      .rpc();
  } catch (error) {
    console.log(error);
  }

  let market_account_info = await program.account.market.fetch(market_account);
  console.log(market_account_info);

  console.log("-----------------set operator----------------------");

  const TUNG_operator = new PublicKey(
    "bnSbou4HkzYGX6Aep7FLyUp872BsyZ2597QRAneJMFk"
  );

  try {
    try {
      await program.methods
        .setAuthority({ operator: {} }, [
          TUNG_operator,
          new PublicKey("aGwtDcFXg9FMJ43axF1x1wqeVjPSLHeVGhmgEGgWn16"),
        ])
        .accounts({
          market: market_account,
          operatorAccount: operator_account,
          adminAccount: admin_account,
        })
        .rpc();
    } catch (error) {
      console.log(error);
    }
  } catch (error) {
    console.log(error);
  }

  //   console.log("-----------------------Listing--------------------------");
  //   const MINT = new PublicKey("5Et3fqFdXqKRKnTvNq8YBrdYWfQdSALJFYiCsjKdHAL7");
  //   const operator = new PublicKey("aGwtDcFXg9FMJ43axF1x1wqeVjPSLHeVGhmgEGgWn16");

  //   const mint_from = await getAssociatedTokenAddress(MINT, operator);

  //   let [listing_mint] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("LISTING_ACCOUNT"), MINT.toBuffer()],
  //     program.programId
  //   );

  //   let market_mint = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     payer,
  //     MINT,
  //     market_account,
  //     true
  //   );

  //   console.log("Market: ", market_account.toString());

  //   try {
  //     await program.methods
  //       .listing(USDC, new anchor.BN(100))
  //       .accounts({
  //         market: market_account,
  //         operatorAccount: operator_account,
  //         listingAccount: listing_mint,
  //         from: mint_from,
  //         to: market_mint.address,
  //         mint: MINT,
  //       })
  //       .rpc();
  //   } catch (error) {
  //     console.log(error);
  //   }

  //   market_account_info = await program.account.market.fetch(market_account);
  //   console.log(market_account_info);
}

init();
