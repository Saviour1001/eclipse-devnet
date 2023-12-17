import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ToDoList } from "../target/types/to_do_list";

describe("to-do-list", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ToDoList as Program<ToDoList>;

  it("Adding a task", async () => {
    const randomTask = anchor.web3.Keypair.generate();

    const tx = await program.methods
      .addTask("My first task")
      .accounts({
        task: randomTask.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([randomTask])
      .rpc();
    console.log("Your transaction signature", tx);

    const task = await program.account.task.fetch(randomTask.publicKey);

    console.log("Your task", task.description);
  });
});
