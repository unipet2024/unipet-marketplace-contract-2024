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

describe("market", async () => {
  let provider = anchor.AnchorProvider.env();

  let owner = provider.wallet as Wallet;
  const payer = owner.payer;
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.Market as Program<Market>;
  let conn = program.provider.connection;

  let currency1, currency2, currency3;
  let seller1, seller2, seller3;
  let seller1Key, seller2Key, seller3Key, operatorKey;
  let new_operator;
  let buyer1, buyer2, buyer3;
  let buyer1Key, buyer2Key, buyer3Key;
  let market_account, admin_account, operator_account;

  beforeEach(async () => {
    // console.log("Initialising...");
    seller1 = new anchor.web3.Keypair();
    seller1Key = seller1.publicKey;
    seller2 = new anchor.web3.Keypair();
    seller2Key = seller2.publicKey;
    seller3 = new anchor.web3.Keypair();
    seller3Key = seller3.publicKey;
    buyer1 = new anchor.web3.Keypair();
    buyer1Key = buyer1.publicKey;
    buyer2 = new anchor.web3.Keypair();
    buyer2Key = buyer2.publicKey;
    buyer3 = new anchor.web3.Keypair();
    buyer3Key = buyer3.publicKey;
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

    currency2 = await createMint(
      program.provider.connection,
      owner.payer,
      owner.publicKey,
      null,
      0
    );

    currency3 = await createMint(
      program.provider.connection,
      owner.payer,
      owner.publicKey,
      null,
      0
    );

    await program.methods
      .init(
        new anchor.BN(1),
        { currency: [currency1, currency2] },
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

    let mint2 = await createMint(conn, payer, owner.publicKey, null, 0);
    console.log("Mint 2: ", mint2.toString());

    let mint3 = await createMint(conn, payer, owner.publicKey, null, 0);
    console.log("Mint 3: ", mint3.toString());

    let market_mint1 = await getOrCreateAssociatedTokenAccount(
      conn,
      payer,
      mint1,
      market_account,
      true
    );

    let market_curr1 = await getOrCreateAssociatedTokenAccount(
      conn,
      payer,
      currency1,
      market_account,
      true
    );

    let market_mint2 = await getOrCreateAssociatedTokenAccount(
      conn,
      payer,
      mint2,
      market_account,
      true
    );

    let market_curr2 = await getOrCreateAssociatedTokenAccount(
      conn,
      payer,
      currency2,
      market_account,
      true
    );

    let market_mint3 = await getOrCreateAssociatedTokenAccount(
      conn,
      payer,
      mint3,
      market_account,
      true
    );

    let market_curr3 = await getOrCreateAssociatedTokenAccount(
      conn,
      payer,
      currency3,
      market_account,
      true
    );

    console.log("Market mint 1 : ", market_mint1.address.toString());

    let seller1_mint1 = await createAta(conn, payer, mint1, seller1.publicKey);
    console.log("Seller 1 mint 1: ", seller1_mint1.toString());

    let seller1_curr1 = await createAta(conn, payer, currency1, seller1Key);
    console.log("Seller 1 currency 1: ", seller1_curr1.toString());

    let buyer1_mint1 = await createAta(conn, payer, mint1, buyer1Key);
    console.log("Buyer 1 mint 1: ", buyer1_mint1.toString());

    let buyer2_mint2 = await createAta(conn, payer, mint2, buyer2Key);
    console.log("Buyer 2 mint 2: ", buyer2_mint2.toString());

    let buyer1_curr1 = await createAta(conn, payer, currency1, buyer1Key);
    console.log("Buyer 1 currency 1: ", buyer1_curr1.toString());

    let buyer2_curr2 = await createAta(conn, payer, currency2, buyer2Key);
    console.log("Buyer 2 currency 2: ", buyer2_curr2.toString());

    let buyer3_curr3 = await createAta(conn, payer, currency3, buyer3Key);
    console.log("Buyer 3 currency 3: ", buyer3_curr3.toString());

    let seller2_mint2 = await createAta(conn, payer, mint2, seller2.publicKey);
    console.log("Seller 2 mint 2: ", seller2_mint2.toString());

    let seller2_curr2 = await createAta(conn, payer, currency2, seller2Key);
    console.log("Seller 2 currency 2: ", seller2_curr2.toString());

    let seller3_mint3 = await createAta(conn, payer, mint3, seller3.publicKey);
    console.log("Seller 3 mint 3: ", seller3_mint3.toString());

    let seller3_curr3 = await createAta(conn, payer, currency3, seller3Key);
    console.log("Seller  currency 3: ", seller3_curr3.toString());

    let operator_mint1 = await createAta(
      conn,
      payer,
      mint1,
      new_operator.publicKey
    );
    console.log("Operator mint 1: ", operator_mint1.toString());

    let operator_curr1 = await createAta(conn, payer, currency1, operatorKey);

    console.log("Mint NFT");
    // await mintTo(conn, owner.payer, mint1, seller1_mint1, owner.payer, 1);
    await mintTo(conn, owner.payer, mint2, seller2_mint2, owner.payer, 1);
    await mintTo(conn, owner.payer, mint3, seller3_mint3, owner.payer, 1);
    await mintTo(conn, owner.payer, mint1, operator_mint1, owner.payer, 1);

    console.log("Mint currency");
    await mintTo(conn, payer, currency1, seller1_curr1, owner.payer, 1000);
    await mintTo(conn, payer, currency2, seller2_curr2, owner.payer, 1000);
    await mintTo(conn, payer, currency3, seller3_curr3, owner.payer, 1000);

    let seller1_mint1_balance =
      await program.provider.connection.getTokenAccountBalance(seller1_mint1);
    console.log("seller1_mint1_balance: ", seller1_mint1_balance.value.amount);

    // console.log("Set new operator");
    // await program.methods
    //   .setOperator(new_operator.publicKey)
    //   .accounts({
    //     market: market_account,
    //     adminAccount: admin_account,
    //     operatorAccount: operator_account,
    //   })
    //   .rpc();

    // let operator_account_info = await program.account.authorityRole.fetch(
    //   operator_account
    // );
    // assert.equal(
    //   operator_account_info.authority.toString(),
    //   new_operator.publicKey.toString(),
    //   "Operator invalid"
    // );

    console.log("Set market to private, only operator can listing");
    await program.methods
      .setStatus({ public: {} })
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

    console.log("Listing mint 1: ", listing_mint1.toString());

    let [listing_mint2] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("LISTING_ACCOUNT"), mint2.toBuffer()],
      program.programId
    );

    console.log("Listing mint 2: ", listing_mint2.toString());

    let [listing_mint3] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("LISTING_ACCOUNT"), mint3.toBuffer()],
      program.programId
    );

    console.log("Listing mint 3: ", listing_mint3.toString());

    console.log("Airdrop");
    await airdrop(conn, owner, seller1.publicKey);
    await airdrop(conn, owner, seller2.publicKey);
    await airdrop(conn, owner, seller3.publicKey);
    await airdrop(conn, owner, buyer1.publicKey);
    await airdrop(conn, owner, buyer2.publicKey);
    await airdrop(conn, owner, buyer3.publicKey);
    await airdrop(conn, owner, new_operator.publicKey);

    // const transaction = await SystemProgram.transfer({
    //   fromPubkey: owner.publicKey,
    //   toPubkey: seller1.publicKey,
    //   lamports: 10 * LAMPORTS_PER_SOL,
    // });

    // const signature = await sendAndConfirmTransaction(
    //   program.provider.connection,
    //   transaction,
    //   [owner],
    // );

    let seller1_balance = await conn.getBalance(seller1.publicKey);
    console.log("Seller 1 balance: ", seller1_balance);

    console.log("**********************************");
    console.log("Operator listing NFT 1");
    try {
      await program.methods
        .listing(currency1, new anchor.BN(100))
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

    assert.equal(listing_mint1_info.currency, currency1.toString());
    assert.equal(
      listing_mint1_info.owner.toString(),
      new_operator.publicKey.toString()
    );
    assert.equal(listing_mint1_info.price.toNumber(), 100);

    console.log("Current time: ", new Date().getTime());
    console.log("Waiting 1 second");

    await setTimeout(1000);
    console.log("Current time: ", new Date().getTime());

    seller1_mint1_balance = await conn.getTokenAccountBalance(seller1_mint1);
    console.log(
      "Seller 1 before buy NFT 1: ",
      seller1_mint1_balance.value.amount
    );

    console.log("Sellet 1 buy NFT 1");
    try {
      await program.methods
        .buyWithSpl()
        .accounts({
          market: market_account,
          currencyFrom: seller1_curr1,
          currencyMarket: market_curr1.address,
          currencyMint: currency1,
          currencyTo: operator_curr1,
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

    let operator_curr1_info = await conn.getTokenAccountBalance(operator_curr1);
    console.log(
      "Operator currency 1 balance: ",
      operator_curr1_info.value.amount
    );

    let market_curr1_info = await conn.getTokenAccountBalance(
      market_curr1.address
    );
    console.log("Market currency 1 balance: ", market_curr1_info.value.amount);

    let seller1_curr1_info = await conn.getTokenAccountBalance(seller1_curr1);
    console.log(
      "Seller 1 currency 1 balance: ",
      seller1_curr1_info.value.amount
    );

    seller1_mint1_balance = await conn.getTokenAccountBalance(seller1_mint1);
    console.log(
      "Seller 1 after buy NFT 1: ",
      seller1_mint1_balance.value.amount
    );

    listing_mint1_info = await program.account.listingData.fetch(listing_mint1);

    // console.log(listing_mint1_info);

    console.log("Admin set market to public");
    await program.methods
      .setStatus({ public: {} })
      .accounts({
        market: market_account,
        operatorAccount: operator_account,
        adminAccount: admin_account,
      })
      .rpc();

    console.log("**********************************");
    console.log("Seller 1 listing NFT 1");
    try {
      await program.methods
        .listing(currency1, new anchor.BN(200))
        .accounts({
          market: market_account,
          operatorAccount: operator_account,
          from: seller1_mint1,
          to: market_mint1.address,
          mint: mint1,
          authority: seller1Key,
          listingAccount: listing_mint1,
        })
        .signers([seller1])
        .rpc();
    } catch (error) {
      console.log(error);
    }

    listing_mint1_info = await program.account.listingData.fetch(listing_mint1);
    assert.equal(listing_mint1_info.price.toNumber(), 200);
    assert.equal(listing_mint1_info.owner.toString(), seller1Key.toString());
    console.log("Listing time: ", listing_mint1_info.listingtime.toNumber());
    console.log("Open time", listing_mint1_info.opentime.toNumber());

    console.log("Current time: ", new Date().getTime());

    console.log("Buyer 1 buy NFT 1");

    try {
      await program.methods
        .buyWithSpl()
        .accounts({
          market: market_account,
          currencyFrom: buyer1_curr1,
          currencyMarket: market_curr1.address,
          currencyMint: currency1,
          currencyTo: seller1_curr1,
          nftTo: buyer1_mint1,
          nftFrom: market_mint1.address,
          nftMint: mint1,
          listingAccount: listing_mint1,
          seller: seller1Key,
          buyer: buyer1Key,
        })
        .signers([buyer1])
        .rpc();
    } catch (error) {
      assert.isTrue(error instanceof AnchorError);
      const err: AnchorError = error;
      const errMsg = "Insufficient amount";
      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log("Error number:", err.error.errorCode.number);
    }

    console.log("Mint currency 1 to buyer 1");
    await mintTo(conn, payer, currency1, buyer1_curr1, owner.payer, 1000);

    // try {
    //   await program.methods
    //     .buyWithSpl()
    //     .accounts({
    //       market: market_account,
    //       currencyFrom: buyer1_curr1,
    //       currencyMarket: market_curr1.address,
    //       currencyMint: currency1,
    //       currencyTo: seller1_curr1,
    //       nftTo: buyer1_mint1,
    //       nftFrom: market_mint1.address,
    //       nftMint: mint1,
    //       listingAccount: listing_mint1,
    //       seller: seller1Key,
    //       buyer: buyer1Key,
    //     })
    //     .signers([buyer1])
    //     .rpc();
    // } catch (error) {
    //   assert.isTrue(error instanceof AnchorError);
    //   const err: AnchorError = error;
    //   const errMsg = "Item still lock";
    //   assert.strictEqual(err.error.errorMessage, errMsg);
    //   console.log("Error number:", err.error.errorCode.number);
    //   // console.log("Error in 491");
    // }

    await setTimeout(1000);

    // listing_mint1_info = await program.account.listingData.fetch(listing_mint1);
    // assert.equal(listing_mint1_info.price.toNumber(), 200);
    // assert.equal(listing_mint1_info.owner.toString(), seller1Key.toString());
    // console.log(listing_mint1_info);

    try {
      await program.methods
        .buyWithSpl()
        .accounts({
          market: market_account,
          currencyFrom: buyer1_curr1,
          currencyMarket: market_curr1.address,
          currencyMint: currency1,
          currencyTo: seller1_curr1,
          nftTo: buyer1_mint1,
          nftFrom: market_mint1.address,
          nftMint: mint1,
          listingAccount: listing_mint1,
          seller: seller1Key,
          buyer: buyer1Key,
        })
        .signers([buyer1])
        .rpc();
    } catch (error) {
      console.log(error);
    }

    seller1_curr1_info = await conn.getTokenAccountBalance(seller1_curr1);
    console.log(
      "Seller 1 currency 1 balance: ",
      seller1_curr1_info.value.amount
    );

    seller1_mint1_balance = await conn.getTokenAccountBalance(seller1_mint1);
    console.log(
      "Seller 1 after selling NFT 1: ",
      seller1_mint1_balance.value.amount
    );

    let buyer1_curr1_info = await conn.getTokenAccountBalance(buyer1_curr1);
    console.log("Buyer 1 curre 1 balance: ", buyer1_curr1_info.value.amount);
    let buyer1_mint1_info = await conn.getTokenAccountBalance(buyer1_mint1);
    console.log("Buyer 1 mint 1 balance: ", buyer1_mint1_info.value.amount);

    market_curr1_info = await conn.getTokenAccountBalance(market_curr1.address);
    console.log("Market curr 1 balance: ", market_curr1_info.value.amount);

    console.log("**********************************");
    console.log("Seller 2 listing NFT 2");
    try {
      await program.methods
        .listing(currency2, new anchor.BN(300))
        .accounts({
          market: market_account,
          operatorAccount: operator_account,
          from: seller2_mint2,
          to: market_mint2.address,
          mint: mint2,
          authority: seller2Key,
          listingAccount: listing_mint2,
        })
        .signers([seller2])
        .rpc();
    } catch (error) {
      console.log(error);
    }

    let listing_mint2_info = await program.account.listingData.fetch(
      listing_mint2
    );
    assert.equal(listing_mint2_info.currency.toString(), currency2.toString());
    assert.equal(listing_mint2_info.price.toNumber(), 300);
    assert.equal(listing_mint2_info.owner.toString(), seller2Key.toString());
    console.log(listing_mint2_info.status);

    console.log("Mint currency 2 to buyer 2");
    await mintTo(conn, payer, currency2, buyer2_curr2, owner.payer, 1000);
    await setTimeout(1000);
    console.log("**********************************");
    console.log("Buyer 2 buy NFT 2");
    try {
      await program.methods
        .buyWithSpl()
        .accounts({
          market: market_account,
          currencyFrom: buyer2_curr2,
          currencyMarket: market_curr2.address,
          currencyMint: currency2,
          currencyTo: seller2_curr2,
          nftTo: buyer2_mint2,
          nftFrom: market_mint2.address,
          nftMint: mint2,
          listingAccount: listing_mint2,
          seller: seller2Key,
          buyer: buyer2Key,
        })
        .signers([buyer2])
        .rpc();
    } catch (error) {
      // console.log(error);
    }
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
