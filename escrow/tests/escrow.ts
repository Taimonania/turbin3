import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { seed } from "@coral-xyz/anchor/dist/cjs/idl";
import { BN } from "bn.js";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Escrow as Program<Escrow>;

  it("can make an escrow", async () => {
    // Add your test here.
    const tx = await program.methods
      .make(new BN(1), new BN(100), new BN(100))
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
