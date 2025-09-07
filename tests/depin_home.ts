import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DepinHome } from "../target/types/depin_home";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAccount,
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { expect } from "chai";

// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());

const provider = anchor.getProvider() as anchor.AnchorProvider;
const program = anchor.workspace.DepinHome as Program<DepinHome>;
const connection = provider.connection;

let rewardsMint: anchor.web3.PublicKey;
let config: anchor.web3.PublicKey;
let userConfig: anchor.web3.PublicKey;

const admin = provider.wallet as anchor.Wallet;
const user = anchor.web3.Keypair.generate();

async function commonSetup() {
  config = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  )[0];
  userConfig = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user"), user.publicKey.toBuffer()],
    program.programId
  )[0];

  rewardsMint = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("rewards")],
    program.programId
  )[0];
  await connection.requestAirdrop(
    user.publicKey,
    2 * anchor.web3.LAMPORTS_PER_SOL
  );
}

describe("Initialize config, Initialize user config, set temperature, set noise, claim reward", () => {
  before(async () => {
    await commonSetup();
  });

  it("initializes config", async () => {
    const tx = await program.methods
      .initialize(3, 5)
      .accountsPartial({
        admin: admin.publicKey,
        config,
        rewardsMint,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([admin.payer])
      .rpc();

    let configAccount = await program.account.config.fetch(config);
    expect(configAccount.rewardAmountNoise).to.equal(5);
    expect(configAccount.rewardAmountTemp).to.equal(3);
    console.log(configAccount);
  });

  it("initializes user-config", async () => {
    const tx = await program.methods
      .initializeUser()
      .accountsPartial({
        user: user.publicKey,
        userConfig,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    let userConfigAccount = await program.account.userConfig.fetch(userConfig);
    expect(userConfigAccount.points).to.equal(0);
    expect(userConfigAccount.noiseDataPoints).to.equal(0);
  });

  it("create a temp data pda", async () => {
    let userAccount = await program.account.userConfig.fetch(userConfig);
    const currentCount = userAccount.tempDataCount;
    const countBuffer = Buffer.alloc(4); // assuming u32

    countBuffer.writeUInt32LE(currentCount, 0);

    const temp = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("temp"), user.publicKey.toBuffer(), countBuffer],
      program.programId
    )[0];
    const tx = await program.methods
      .setTemp(30)
      .accountsPartial({
        user: user.publicKey,
        temp,
        userConfig,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
    userAccount = await program.account.userConfig.fetch(userConfig);
    let tempAccount = await program.account.temp.fetch(temp);
    expect(tempAccount.value).to.equal(30);
    expect(userAccount.tempDataPoints).to.equal(1);
    expect(userAccount.tempDataCount).to.equal(1);
  });

  it("create a second temp data pda", async () => {
    let userAccount = await program.account.userConfig.fetch(userConfig);
    const currentCount = userAccount.tempDataCount;
    const countBuffer = Buffer.alloc(4); // assuming u32

    countBuffer.writeUInt32LE(currentCount, 0);

    const temp = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("temp"), user.publicKey.toBuffer(), countBuffer],
      program.programId
    )[0];
    const tx = await program.methods
      .setTemp(30)
      .accountsPartial({
        user: user.publicKey,
        temp,
        userConfig,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
    userAccount = await program.account.userConfig.fetch(userConfig);
    let tempAccount = await program.account.temp.fetch(temp);
    expect(tempAccount.value).to.equal(30);
    expect(userAccount.tempDataPoints).to.equal(2);
    expect(userAccount.tempDataCount).to.equal(2);
  });

  it("create a noise data pda", async () => {
    let userAccount = await program.account.userConfig.fetch(userConfig);
    const currentCount = userAccount.noiseDataCount;
    const countBuffer = Buffer.alloc(4); // assuming u32

    countBuffer.writeUInt32LE(currentCount, 0);
    const noise = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("noise"), user.publicKey.toBuffer(), countBuffer],
      program.programId
    )[0];

    const tx = await program.methods
      .setNoise(30)
      .accountsPartial({
        user: user.publicKey,
        noise,
        userConfig,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
    userAccount = await program.account.userConfig.fetch(userConfig);
    let noiseAccount = await program.account.noise.fetch(noise);
    expect(noiseAccount.value).to.equal(30);
    expect(userAccount.noiseDataPoints).to.equal(1);
    expect(userAccount.noiseDataCount).to.equal(1);
    // console.log(userAccount);
  });

  it("lets user claim the reward tokens, set user points to zero", async () => {
    let configAc = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    )[0];
    const rewardsAta = getAssociatedTokenAddressSync(
      rewardsMint,
      user.publicKey
    );
    const tx = await program.methods
      .claim()
      .accountsPartial({
        user: user.publicKey,
        config: configAc,
        userConfig,
        rewardsMint,
        rewardsAta,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();

    const rewardsAccount = await getAccount(connection, rewardsAta);
    let userAccount = await program.account.userConfig.fetch(userConfig);
    expect(userAccount.tempDataPoints).to.equal(0);
    expect(userAccount.tempDataCount).to.equal(2);
    expect(userAccount.noiseDataCount).to.equal(1);
    expect(Number(rewardsAccount.amount.toString()) / 1000_000).to.equal(11);
  });
});
