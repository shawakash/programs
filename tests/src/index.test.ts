import { expect, test } from "bun:test";
import * as borsh from "borsh";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import {
  ACCOUNT_DATA_LENGTH,
  AccountData,
  accountDataSchema,
  CalcTypeEnum,
  InstructionType,
  instructionTypeSchema,
} from "./types";

const adminAccount = Keypair.generate();
const dataAccount = new Keypair();

const programId = new PublicKey("FR7EirJHK94Z38Xu87HPWTnD56jMLEwnu3YeNG7CpoDq");
const connection = new Connection("http://127.0.0.1:8899", "confirmed");

test("Account is initialized", async () => {
  const adminAirdropTxn = await connection.requestAirdrop(
    adminAccount.publicKey,
    200 * LAMPORTS_PER_SOL,
  );
  await connection.confirmTransaction(adminAirdropTxn);

  const lamports =
    await connection.getMinimumBalanceForRentExemption(ACCOUNT_DATA_LENGTH);

  const ix = SystemProgram.createAccount({
    fromPubkey: adminAccount.publicKey,
    lamports,
    newAccountPubkey: dataAccount.publicKey,
    programId,
    space: ACCOUNT_DATA_LENGTH,
  });

  const txn = new Transaction();
  txn.add(ix);

  const txnHash = await connection.sendTransaction(txn, [
    adminAccount,
    dataAccount,
  ]);
  await connection.confirmTransaction(txnHash);
});

test("Expect the data account calc_val to be 0", async () => {
  const info = await connection.getAccountInfo(dataAccount.publicKey);
  if (!info) {
    throw new Error("Account not found");
  }
  expect(info.lamports).toBeGreaterThan(0);
  const data = borsh.deserialize(accountDataSchema, info.data) as AccountData;
  expect(data.calc_val).toBe(0);
});

test("Perfom increment", async () => {
  const instructionData = new InstructionType(CalcTypeEnum.Increment, 20);

  const ixData = borsh.serialize(instructionTypeSchema, instructionData);
  const ix = new TransactionInstruction({
    keys: [
      {
        pubkey: dataAccount.publicKey,
        isSigner: true,
        isWritable: true,
      },
    ],
    programId,
    data: Buffer.from(ixData),
  });

  const txn = new Transaction();
  txn.add(ix);
  const txnHash = await connection.sendTransaction(txn, [
    adminAccount,
    dataAccount,
  ]);
  await connection.confirmTransaction(txnHash);

  const info = await connection.getAccountInfo(dataAccount.publicKey);
  if (!info) {
    throw new Error("Account not found");
  }
  expect(info.lamports).toBeGreaterThan(0);
  const data = borsh.deserialize(accountDataSchema, info.data) as AccountData;
  expect(data.calc_val).toBe(20);
});

test("Perfom decrement", async () => {
  const instructionData = new InstructionType(CalcTypeEnum.Decrement, 10);

  const ixData = borsh.serialize(instructionTypeSchema, instructionData);
  const ix = new TransactionInstruction({
    keys: [
      {
        pubkey: dataAccount.publicKey,
        isSigner: true,
        isWritable: true,
      },
    ],
    programId,
    data: Buffer.from(ixData),
  });

  const txn = new Transaction();
  txn.add(ix);
  const txnHash = await connection.sendTransaction(txn, [
    adminAccount,
    dataAccount,
  ]);
  await connection.confirmTransaction(txnHash);

  const info = await connection.getAccountInfo(dataAccount.publicKey);
  if (!info) {
    throw new Error("Account not found");
  }
  expect(info.lamports).toBeGreaterThan(0);
  const data = borsh.deserialize(accountDataSchema, info.data) as AccountData;
  expect(data.calc_val).toBe(20 - 10);
});

test("Perfom mutiply", async () => {
  const instructionData = new InstructionType(CalcTypeEnum.Multiply, 10);

  const ixData = borsh.serialize(instructionTypeSchema, instructionData);
  const ix = new TransactionInstruction({
    keys: [
      {
        pubkey: dataAccount.publicKey,
        isSigner: true,
        isWritable: true,
      },
    ],
    programId,
    data: Buffer.from(ixData),
  });

  const txn = new Transaction();
  txn.add(ix);
  const txnHash = await connection.sendTransaction(txn, [
    adminAccount,
    dataAccount,
  ]);
  await connection.confirmTransaction(txnHash);

  const info = await connection.getAccountInfo(dataAccount.publicKey);
  if (!info) {
    throw new Error("Account not found");
  }
  expect(info.lamports).toBeGreaterThan(0);
  const data = borsh.deserialize(accountDataSchema, info.data) as AccountData;
  expect(data.calc_val).toBe(10 * 10);
});

test("Perfom divide", async () => {
  const instructionData = new InstructionType(CalcTypeEnum.Divide, 10);

  const ixData = borsh.serialize(instructionTypeSchema, instructionData);
  const ix = new TransactionInstruction({
    keys: [
      {
        pubkey: dataAccount.publicKey,
        isSigner: true,
        isWritable: true,
      },
    ],
    programId,
    data: Buffer.from(ixData),
  });

  const txn = new Transaction();
  txn.add(ix);
  const txnHash = await connection.sendTransaction(txn, [
    adminAccount,
    dataAccount,
  ]);
  await connection.confirmTransaction(txnHash);

  const info = await connection.getAccountInfo(dataAccount.publicKey);
  if (!info) {
    throw new Error("Account not found");
  }
  expect(info.lamports).toBeGreaterThan(0);
  const data = borsh.deserialize(accountDataSchema, info.data) as AccountData;
  expect(data.calc_val).toBe(100 / 10);
});
