import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaDapp } from "../target/types/solana_dapp";
import * as assert from "assert";
import * as bs58 from "bs58";

describe("solana-dapp", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaDapp as Program<SolanaDapp>;

  it("can send a new message", async () => {
    // Add your test here.
    // Create a new account to store the message
    const account = anchor.web3.Keypair.generate();

    const solanaTopic = "Topic for Solana Dapp";
    const solanaContent = "Content for Solana Dapp";

    // Invoke the program's `sendMessage` method to send a new message
    await program.rpc.sendMessage(solanaTopic, solanaContent, {
      accounts: {
        message: account.publicKey,
        author: program.provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [account],
    });

    // Fetch the account data to verify the message
    const accountData = await program.account.message.fetch(account.publicKey);

    // Perform the assertions
    assert.equal(accountData.topic, solanaTopic);
    assert.equal(accountData.content, solanaContent);
    assert.equal(accountData.author.toBase58(), program.provider.publicKey.toBase58());
    assert.ok(accountData.timestamp);
  });
});
