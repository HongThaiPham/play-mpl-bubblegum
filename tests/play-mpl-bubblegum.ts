import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { PlayMplBubblegum } from "../target/types/play_mpl_bubblegum";

import { getMerkleTreeSize } from "@metaplex-foundation/spl-account-compression";

describe("play-mpl-bubblegum", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .playMplBubblegum as Program<PlayMplBubblegum>;

  const maxDepth = 3;
  const maxBufferSize = 8;
  const isPublic = true; // Set to true for public trees
  const merkleTree = web3.Keypair.generate();
  const collectionMint = web3.Keypair.generate();

  it("init merkle tree", async () => {
    const merkleTreeSize = getMerkleTreeSize(maxDepth, maxBufferSize);

    let initTreeInstruction = anchor.web3.SystemProgram.createAccount({
      fromPubkey: provider.publicKey,
      newAccountPubkey: merkleTree.publicKey,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(
        merkleTreeSize
      ),
      space: merkleTreeSize,
      programId: new web3.PublicKey(
        "mcmt6YrQEMKw8Mw43FmpRLmf7BqRnFMKmAcbxE3xkAW"
      ),
    });
    // Add your test here.
    const tx = await program.methods
      .createTree(maxDepth, maxBufferSize, isPublic)
      .accountsPartial({
        merkleTree: merkleTree.publicKey,
      })
      .preInstructions([initTreeInstruction])
      .signers([merkleTree])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("mint nft", async () => {
    const tx = await program.methods
      .mintNft()
      .accounts({
        merkleTree: merkleTree.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("create collection", async () => {
    const tx = await program.methods
      .createCollection("NFT Collection", "https://example.com")
      .accounts({
        collection: collectionMint.publicKey,
      })
      .signers([collectionMint, collectionMint])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
