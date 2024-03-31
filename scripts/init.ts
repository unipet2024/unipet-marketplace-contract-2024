import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider } from "@coral-xyz/anchor";
import { IDL } from "../target/types/market";
import { Wallet } from "@coral-xyz/anchor";
import * as privatekey from '/Users/tabatrung/.config/solana/id.json'
// import { setTimeout } from "timers/promises";

import { PublicKey, Keypair, Connection, clusterApiUrl } from "@solana/web3.js";
const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
const wallet = Keypair.fromSecretKey(
  Uint8Array.from([
    60, 31, 216, 134, 68, 78, 5, 54, 175, 135, 221, 227, 168, 70, 131, 114, 133,
    65, 139, 93, 195, 126, 28, 32, 17, 15, 252, 196, 1, 237, 44, 57, 8, 134, 50,
    123, 56, 199, 184, 99, 61, 162, 196, 68, 143, 51, 117, 64, 26, 54, 84, 218,
    154, 157, 209, 231, 34, 3, 251, 190, 216, 153, 90, 113,
  ])
);
console.log("Wallet:", wallet.publicKey.toString());

new Wallet(wallet);

const provider = new AnchorProvider(
  connection,
  new Wallet(wallet),
  anchor.AnchorProvider.defaultOptions()
);
// console.log("Provider: ", provider);

const idl = IDL;
// Address of the deployed program.
const programId = "7ScnRwX7fYPQbc126PPtMYdgHSE9zhbXLAcYY6rqgAEx";
// Generate the program client from IDL.
const program = new anchor.Program(idl, programId, provider);

async function init() {
  let owner = provider.wallet as Wallet;
  const payer = owner.payer;
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const unp_token = new PublicKey(
    "5Et3fqFdXqKRKnTvNq8YBrdYWfQdSALJFYiCsjKdHAL7"
  );
  const usdc_token = new PublicKey(
    "BUJST4dk6fnM5G3FnhTVc3pjxRJE7w2C5YL9XgLbdsXW"
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

  try {
    await program.methods
      .init(
        new anchor.BN(300),
        { currency: [unp_token, usdc_token] },
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

  let market_account_info = await program.account.market.fetch(market_account);
  console.log(market_account_info);
}

init();
