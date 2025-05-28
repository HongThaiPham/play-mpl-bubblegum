import { Program, web3 } from "@coral-xyz/anchor";
import { getConfig } from "./helpers";
import * as idl from "../target/idl/play_mpl_bubblegum.json";
import { PlayMplBubblegum } from "../target/types/play_mpl_bubblegum";
import {
  address,
  createTransactionMessage,
  generateKeyPairSigner,
  getPublicKeyFromAddress,
  getSignatureFromTransaction,
  pipe,
  prependTransactionMessageInstruction,
  prependTransactionMessageInstructions,
  setTransactionMessageFeePayer,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from "@solana/kit";
import { getMerkleTreeSize } from "@metaplex-foundation/spl-account-compression";
import { getCreateAccountInstruction } from "@solana-program/system";
import { fromLegacyTransactionInstruction } from "@solana/compat";

import {
  fetchTreeConfigFromSeeds,
  findLeafAssetIdPda,
  findTreeConfigPda,
} from "@metaplex-foundation/mpl-bubblegum";
import { fromWeb3JsPublicKey } from "@metaplex-foundation/umi-web3js-adapters";

const maxDepth = 3;
const maxBufferSize = 8;

(async () => {
  const { payer, rpc, sendAndConfirmTransaction, provider, umi } =
    await getConfig();

  const merkleTree = await generateKeyPairSigner();
  const merkleTreePublicKey = fromWeb3JsPublicKey(
    new web3.PublicKey(merkleTree.address)
  );
  const treeConfigPda = findTreeConfigPda(umi, {
    merkleTree: merkleTreePublicKey,
  });

  const program = new Program<PlayMplBubblegum>(idl, provider);

  {
    const merkleTreeSize = getMerkleTreeSize(maxDepth, maxBufferSize);

    let initTreeInstruction = getCreateAccountInstruction({
      payer: payer,
      newAccount: merkleTree,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(
        merkleTreeSize
      ),
      space: merkleTreeSize,
      programAddress: address("mcmt6YrQEMKw8Mw43FmpRLmf7BqRnFMKmAcbxE3xkAW"),
    });

    const createTreeInstruction = await program.methods
      .createTree(maxDepth, maxBufferSize)
      .accountsPartial({
        merkleTree: merkleTree.address,
        noopProgram: address("mnoopTCrg4p8ry25e4bcWA9XZjbNjMTfgYVGGEdRsf3"),
      })
      .signers([merkleTree.keyPair])
      .instruction();

    let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

    const transactionMessage = pipe(
      createTransactionMessage({
        version: 0,
      }),
      (tx) => setTransactionMessageFeePayer(payer.address, tx),
      (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
      (tx) =>
        prependTransactionMessageInstructions(
          [
            initTreeInstruction,
            fromLegacyTransactionInstruction(createTreeInstruction),
          ],
          tx
        )
    );

    const signedTransaction = await signTransactionMessageWithSigners(
      transactionMessage
    );

    console.info({ signature: getSignatureFromTransaction(signedTransaction) });

    await sendAndConfirmTransaction(signedTransaction, {
      commitment: "confirmed",
    });
  }

  {
    const mintInstruction = await program.methods
      .mintNft()
      .accountsPartial({
        merkleTree: merkleTree.address,
      })
      .instruction();

    let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

    const transactionMessage = pipe(
      createTransactionMessage({
        version: 0,
      }),
      (tx) => setTransactionMessageFeePayerSigner(payer, tx),
      (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
      (tx) =>
        prependTransactionMessageInstruction(
          fromLegacyTransactionInstruction(mintInstruction),
          tx
        )
    );

    const signedTransaction = await signTransactionMessageWithSigners(
      transactionMessage
    );

    console.info({ signature: getSignatureFromTransaction(signedTransaction) });

    await sendAndConfirmTransaction(signedTransaction, {
      commitment: "confirmed",
    });

    {
      let treeFound = false;

      while (!treeFound) {
        try {
          const treeConfig = await fetchTreeConfigFromSeeds(umi, {
            merkleTree: merkleTreePublicKey,
          });
          treeFound = true;
          console.log(
            `🌲 Merkle Tree created: ${merkleTreePublicKey.toString()}. Config:`
          );
          console.log(
            `     - Total Mint Capacity ${Number(
              treeConfig.totalMintCapacity
            ).toLocaleString()}`
          );
          console.log(
            `     - Number Minted: ${Number(
              treeConfig.numMinted
            ).toLocaleString()}`
          );
          console.log(`     - Is Public: ${treeConfig.isPublic}`);
          console.log(
            `     - Is Decompressible: ${treeConfig.isDecompressible}`
          );
        } catch {
          // If not found yet, wait and retry
          await new Promise((resolve) => setTimeout(resolve, 5000));
        }
      }
    }

    const assetId = findLeafAssetIdPda(umi, {
      merkleTree: merkleTreePublicKey,
      leafIndex: 0,
    });
    console.log(`🍃 NFT Minted: ${assetId[0].toString()}`);
  }
})();
