import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Prog1 } from "../target/types/prog1";
import { PublicKey } from "@solana/web3.js";

describe("prog1", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.Prog1 as Program<Prog1>;

  const wallet = provider.wallet;

  let name = "Mark's journal number 2";

  const [journalPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from(name), wallet.publicKey.toBuffer()],
    program.programId
  );


  it("Creating journal", async () => {
    const tx = await program.methods.initJournal(name, "Hello World!").accounts({journalEntry: journalPDA}).rpc();
    console.log("Journal created successfuly!")
    console.log(`Tx sig ${tx}`);
  });

  it("Updating journal", async () => {
    const tx = await program.methods.updateJournal(name, "Bye Bye World!").accounts({journal: journalPDA}).rpc();
    console.log("Journal updated successfuly!")
    console.log(`Tx sig ${tx}`);
  });

  it("Deleting journal", async () => {
    const tx = await program.methods.deleteJournal(name).accounts({journal: journalPDA}).rpc();
    console.log("Journal deleted successfuly!")
    console.log(`Tx sig ${tx}`);
  });

});
