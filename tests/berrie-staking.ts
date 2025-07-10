import * as anchor from "@coral-xyz/anchor";
import { IdlTypes, Program } from "@coral-xyz/anchor";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { Keypair, PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { config } from "dotenv";
import IDL from "../target/idl/berrie_staking.json";
import { BerrieStaking } from "../target/types/berrie_staking";

config({ path: "./tests/.env" });

const logTxnSignature = (tx: string) => {
  console.log(
    "Your transaction signature",
    `https://explorer.solana.com/tx/${tx}?cluster=devnet`
  );
};

type Global = IdlTypes<BerrieStaking>["global"];
type Stake = IdlTypes<BerrieStaking>["stake"];

const TOKEN_PUBKEY = new PublicKey(
  IDL.constants.find((c) => c.name === "TOKEN_PUBKEY")!.value
);

const ADMIN_PUBKEY = new PublicKey(
  IDL.constants.find((c) => c.name === "ADMIN_PUBKEY")!.value
);

const MIN_STAKE_DURATION = Number(
  IDL.constants.find((c) => c.name === "MIN_STAKE_DURATION")!.value
);

const TOTAL_UNLOCK_DURATION = Number(
  IDL.constants.find((c) => c.name === "TOTAL_UNLOCK_DURATION")!.value
);

const getGlobalAddress = (program: Program<BerrieStaking>) => {
  const GLOBAL_SEED = "global";

  const [globalPublicKey] = PublicKey.findProgramAddressSync(
    [Buffer.from(GLOBAL_SEED)],
    program.programId
  );
  return globalPublicKey;
};

const getGlobalInfo = async (program: Program<BerrieStaking>) => {
  const globalPublicKey = getGlobalAddress(program);

  const globalInfo = await program.account.global.fetch(
    globalPublicKey,
    "confirmed"
  );
  return globalInfo;
};

const getStakeInfos = async (
  program: Program<BerrieStaking>,
  user: PublicKey
) => {
  const stakedInfos = await program.account.stake.all([
    {
      memcmp: {
        offset: 8,
        bytes: user.toBase58(),
      },
    },
  ]);

  return stakedInfos;
};

const getEventInfos = async (
  program: Program<BerrieStaking>,
  user: PublicKey
) => {
  const eventInfos = await program.account.event.all([
    {
      memcmp: {
        offset: 8,
        bytes: user.toBase58(),
      },
    },
  ]);

  return eventInfos.sort(
    (a, b) => b.account.timestamp.toNumber() - a.account.timestamp.toNumber()
  );
};

describe("berrie-staking", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BerrieStaking as Program<BerrieStaking>;

  const adminKeypair = Keypair.fromSecretKey(
    bs58.decode(process.env.ADMIN_PRIVATE_KEY!)
  );
  const admin = adminKeypair.publicKey;

  const userKeypair = Keypair.fromSecretKey(
    bs58.decode(process.env.USER_PRIVATE_KEY!)
  );
  const user = userKeypair.publicKey;

  /**---------------- ADMIN INSTRUCTIONS ----------------*/

  it.skip("Deposit Token", async () => {
    const globalPublicKey = getGlobalAddress(program);
    console.log("Global Address: ", globalPublicKey.toBase58());

    const amount = new BN(1000);

    const tx = await program.methods
      .depositToken(amount)
      .accounts({ admin })
      .rpc();

    logTxnSignature(tx);
  });

  /**---------------- USER INSTRUCTIONS ----------------*/

  it.skip("Stake Token", async () => {
    const amount = new BN(10 ** 10);
    const stakeId = new BN(Math.floor(Date.now() / 1000));
    const eventId = new BN(Math.floor(Date.now() / 1000));

    const tx = await program.methods
      .stakeToken(stakeId, eventId, amount)
      .accounts({
        user,
      })
      .signers([userKeypair])
      .rpc();

    logTxnSignature(tx);
  });

  it.skip("Unstake Token", async () => {
    const stakeId = new BN(1434235253);
    const eventId = new BN(Math.floor(Date.now() / 1000));

    const tx = await program.methods
      .unstakeToken(stakeId, eventId)
      .accounts({
        user,
      })
      .signers([userKeypair])
      .rpc();

    logTxnSignature(tx);
  });

  it.skip("Claim Token", async () => {
    const stakeId = new BN(1434235253);
    const eventId = new BN(Math.floor(Date.now() / 1000));

    const tx = await program.methods
      .claimToken(stakeId, eventId)
      .accounts({
        user,
      })
      .signers([userKeypair])
      .rpc();

    logTxnSignature(tx);
  });
});
