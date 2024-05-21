import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorError } from "@coral-xyz/anchor";
import { Market } from "../target/types/market";
import { Wallet } from "@coral-xyz/anchor";
import { setTimeout } from "timers/promises";

import {
  SystemProgram,
  LAMPORTS_PER_SOL,
  sendAndConfirmRawTransaction,
  Transaction,
  sendAndConfirmTransaction,
  PublicKey,
} from "@solana/web3.js";

import {
  createMint,
  createAssociatedTokenAccount,
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  transfer,
  createAccount,
} from "@solana/spl-token";

import { assert, expect } from "chai";
const address0 = new PublicKey("11111111111111111111111111111111");

describe("market", async () => {
  let provider = anchor.AnchorProvider.env();

  let owner = provider.wallet as Wallet;
  const payer = owner.payer;
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.Market as Program<Market>;
  let conn = program.provider.connection;

  let currency1;
  let seller1;
  let seller1Key, operatorKey;
  let new_operator;
  let buyer1;
  let buyer1Key;
  let market_account, admin_account, operator_account;

  beforeEach(async () => {
    // console.log("Initialising...");
    seller1 = new anchor.web3.Keypair();
    seller1Key = seller1.publicKey;
    buyer1 = new anchor.web3.Keypair();
    buyer1Key = buyer1.publicKey;
    new_operator = new anchor.web3.Keypair();
    operatorKey = new_operator.publicKey;
    console.log("owner", owner.publicKey.toString());

    [market_account] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("MARKET_ACCOUNT")],
      program.programId
    );

    [admin_account] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("ADMIN_ROLE")],
      program.programId
    );

    [operator_account] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("OPERATOR_ROLE")],
      program.programId
    );

    currency1 = await createMint(
      program.provider.connection,
      owner.payer,
      owner.publicKey,
      null,
      0
    );

    await program.methods
      .init(
        new anchor.BN(1),
        { currency: [currency1, address0] },
        new anchor.BN(20)
      )
      .accounts({
        market: market_account,
        adminAccount: admin_account,
        operatorAccount: operator_account,
      })
      .rpc();

    console.log("Admin account: ", admin_account.toString());
    console.log("Market account:", market_account.toString());
  });

  it("Listing!", async () => {
    console.log("Seller 1 listing NFT 1");

    let mint1 = await createMint(conn, payer, owner.publicKey, null, 0);
    console.log("Mint 1: ", mint1.toString());

    let market_mint1 = await getOrCreateAssociatedTokenAccount(
      conn,
      payer,
      mint1,
      market_account,
      true
    );

    let operator_mint1 = await createAta(
      conn,
      payer,
      mint1,
      new_operator.publicKey
    );
    console.log("Operator mint 1: ", operator_mint1.toString());

    let operator_curr1 = await createAta(conn, payer, currency1, operatorKey);

    let seller1_mint1 = await createAta(conn, payer, mint1, seller1.publicKey);
    console.log("Seller 1 mint 1: ", seller1_mint1.toString());

    // let buyer1_mint1 = await createAta(conn, payer, mint1, seller1.publicKey);
    // console.log("Seller 1 mint 1: ", seller1_mint1.toString());

    console.log("Mint NFT");
    await mintTo(conn, owner.payer, mint1, operator_mint1, owner.payer, 1);

    console.log("Set new operator");
    await program.methods
      .setOperator(new_operator.publicKey)
      .accounts({
        market: market_account,
        adminAccount: admin_account,
        operatorAccount: operator_account,
      })
      .rpc();

    let operator_account_info = await program.account.authorityRole.fetch(
      operator_account
    );
    assert.equal(
      operator_account_info.authority.toString(),
      new_operator.publicKey.toString(),
      "Operator invalid"
    );

    console.log("Set market to private, only operator can listing");
    await program.methods
      .setStatus({ private: {} })
      .accounts({
        market: market_account,
        operatorAccount: operator_account,
        adminAccount: admin_account,
      })
      .rpc();

    let [listing_mint1] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("LISTING_ACCOUNT"), mint1.toBuffer()],
      program.programId
    );

    console.log("Airdrop");
    await airdrop(conn, owner, seller1.publicKey);
    await airdrop(conn, owner, buyer1.publicKey);
    await airdrop(conn, owner, new_operator.publicKey);

    let seller1_balance = await conn.getBalance(seller1.publicKey);
    console.log("Seller 1 balance: ", seller1_balance);

    let market_balance = await conn.getBalance(market_account);
    console.log("Market balance: ", market_balance);

    console.log("**********************************");
    console.log("Operator listing NFT 1 with currency is SOL");
    let nft1_price = 1000;
    try {
      await program.methods
        .listing(address0, new anchor.BN(nft1_price))
        .accounts({
          market: market_account,
          operatorAccount: operator_account,
          from: operator_mint1,
          to: market_mint1.address,
          mint: mint1,
          authority: new_operator.publicKey,
          listingAccount: listing_mint1,
        })
        .signers([new_operator])
        .rpc();
    } catch (error) {
      console.log(error);
    }

    let listing_mint1_info = await program.account.listingData.fetch(
      listing_mint1
    );

    console.log(listing_mint1_info.listingtime.toNumber());
    console.log(listing_mint1_info.opentime.toNumber());

    assert.equal(listing_mint1_info.currency.toString(), address0.toString());
    assert.equal(
      listing_mint1_info.owner.toString(),
      new_operator.publicKey.toString()
    );
    assert.equal(listing_mint1_info.price.toNumber(), nft1_price);

    console.log("Current time: ", new Date().getTime());
    console.log("Waiting 1 second");

    await setTimeout(1000);
    console.log("Current time: ", new Date().getTime());

    let buyer1_balance = console.log("Sellet 1 buy NFT 1");
    try {
      await program.methods
        .buyWithSol()
        .accounts({
          market: market_account,
          nftTo: seller1_mint1,
          nftFrom: market_mint1.address,
          nftMint: mint1,
          listingAccount: listing_mint1,
          seller: operatorKey,
          buyer: seller1Key,
        })
        .signers([seller1])
        .rpc();
    } catch (error) {
      console.log(error);
    }

    seller1_balance = await conn.getBalance(seller1.publicKey);
    console.log("Seller 1 balance: ", seller1_balance);
    console.log(LAMPORTS_PER_SOL - nft1_price);
    assert.equal(LAMPORTS_PER_SOL - nft1_price, seller1_balance);

    market_balance = await conn.getBalance(market_account);
    console.log("Market balance: ", market_balance);
    // assert.equal(market_balance, nft1_price);
  });
});

async function airdrop(con, from, to) {
  let transaction = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey: from.publicKey,
      toPubkey: to,
      lamports: LAMPORTS_PER_SOL,
    })
  );

  // Sign transaction, broadcast, and confirm
  await sendAndConfirmTransaction(con, transaction, [from.payer]);
}

async function createAta(conn, payer, mint, to) {
  return await createAssociatedTokenAccount(conn, payer, mint, to);
}

async function getOrCreateAta(conn, payer, mint1, acc) {
  return await getOrCreateAssociatedTokenAccount(conn, payer, mint1, acc, true);
}
