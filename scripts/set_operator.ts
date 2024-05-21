import * as anchor from "@coral-xyz/anchor";
import { Wallet } from "@coral-xyz/anchor";

import { PublicKey } from "@solana/web3.js";

import { program, provider } from "./helper";

async function init() {
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const TUNG_operator = new PublicKey(
    "bnSbou4HkzYGX6Aep7FLyUp872BsyZ2597QRAneJMFk"
  );

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

  // try {
  //   try {
  //     await program.methods
  //       .setAuthority({ operator: {} }, [
  //         TUNG_operator,
  //         new PublicKey("aGwtDcFXg9FMJ43axF1x1wqeVjPSLHeVGhmgEGgWn16"),
  //       ])
  //       .accounts({
  //         market: market_account,
  //         operatorAccount: operator_account,
  //         adminAccount: admin_account,
  //       })
  //       .rpc();
  //   } catch (error) {
  //     console.log(error);
  //   }
  // } catch (error) {
  //   console.log(error);
  // }

  let market_account_info = await program.account.market.fetch(market_account);
  console.log(market_account_info);

  let operator_account_info = await program.account.authorityRole.fetch(
    operator_account
  );
  console.log(operator_account_info);
}

init();
