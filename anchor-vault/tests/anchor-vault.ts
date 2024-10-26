import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { BN } from "bn.js";
import { assert, should } from "chai";

should();

describe("anchor-vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorVault as Program<AnchorVault>;
  const [state] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("state"), program.provider.publicKey.toBuffer()],
    program.programId
  );
  const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), state.toBuffer()],
    program.programId
  );

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().rpc();
  });

  it("Deposits correct amount", async () => {
    const transferAmount = new BN(10000000);
    const tx = await program.methods.deposit(transferAmount).rpc();

    // Fetch the vault's balance after deposit
    const vaultBalance = await program.provider.connection.getBalance(vault);

    // Assert that the vault balance increased by the transfer amount
    const expectedBalance = transferAmount.toNumber();
    vaultBalance.should.equal(
      expectedBalance,
      "Vault balance should match the deposited amount"
    );
  });

  it("Withdraws correct amount", async () => {
    const transferAmount = new BN(5000000);
    const tx = await program.methods.withdraw(transferAmount).rpc();

    // Fetch the vault's balance after withdrawal
    const vaultBalance = await program.provider.connection.getBalance(vault);
    vaultBalance.should.equal(
      5000000,
      "Vault balance should be 0 after withdrawal"
    );
  });

  it("Closes the vault and vault state", async () => {
    const userBalanceBefore = await program.provider.connection.getBalance(
      program.provider.publicKey
    );
    const tx = await program.methods.close().rpc();
    // Fetch the vault's balance after closing
    const vaultBalance = await program.provider.connection.getBalance(vault);
    vaultBalance.should.equal(0, "Vault balance should be 0 after closing");

    // Fetch the user's balance after closing
    const userBalance = await program.provider.connection.getBalance(
      program.provider.publicKey
    );

    // Fetch the rent-exempt minimum for the vault state account
    const rentExemptMinimum =
      await program.provider.connection.getMinimumBalanceForRentExemption(
        program.account.vaultState.size
      );

    // Check if the user's balance increased by at least the rent-exempt minimum
    const userBalanceIncreased =
      userBalance - userBalanceBefore >= rentExemptMinimum;
    userBalanceIncreased.should.equal(
      true,
      "User balance should have increased by at least the rent-exempt minimum"
    );

    // Attempt to fetch the vault state account (should fail as it's closed)
    try {
      await program.account.vaultState.fetch(state);
      throw new Error("Vault state account should not exist");
    } catch (error) {
      error.message.should.include("Account does not exist");
    }
  });
});
