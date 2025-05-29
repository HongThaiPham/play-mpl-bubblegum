import * as anchor from "@coral-xyz/anchor";
import { web3 } from "@coral-xyz/anchor";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { dasApi } from "@metaplex-foundation/digital-asset-standard-api";
import { mplBubblegum } from "@metaplex-foundation/mpl-bubblegum";
import {
  createSignerFromKeypair,
  keypairIdentity,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  sendAndConfirmTransactionFactory,
} from "@solana/kit";

import dotenv from "dotenv";
dotenv.config();

const PAYER_PRIVATE_KEY = process.env.PAYER_PRIVATE_KEY as string;
const RPC_HOST = process.env.RPC_HOST as string;

export async function getConfig() {
  const oldPayer = web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(PAYER_PRIVATE_KEY))
  );
  const payer = await createKeyPairSignerFromBytes(
    new Uint8Array(JSON.parse(PAYER_PRIVATE_KEY))
  );

  const user1 = await createKeyPairSignerFromBytes(
    new Uint8Array(
      JSON.parse(
        require("fs").readFileSync(
          "./test-wallet/Cns1AfgvwRHPwWqEZkfwJncJs7Y3emhuygGexQofpsNv.json"
        )
      )
    )
  );

  const user2 = await createKeyPairSignerFromBytes(
    new Uint8Array(
      JSON.parse(
        require("fs").readFileSync(
          "./test-wallet/MTWGugyrtQj6C3ZsPxECTNjiSk4FdaA8bnGbcigjRSQ.json"
        )
      )
    )
  );

  const connection = new web3.Connection(`https://${RPC_HOST}`);
  const wallet = new NodeWallet(oldPayer);
  const payerPublicKey = new web3.PublicKey(payer.address);
  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "processed",
  });

  const rpc = createSolanaRpc(`https://${RPC_HOST}`);
  const rpcSubscriptions = createSolanaRpcSubscriptions(`wss://${RPC_HOST}`);

  const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({
    /**
     * The RPC implements a `sendTransaction` method which relays transactions to the network.
     */
    rpc,
    /**
     * RPC subscriptions allow the transaction sender to subscribe to the status of our transaction.
     * The sender will resolve when the transaction is reported to have been confirmed, or will
     * reject in the event of an error, or a timeout if the transaction lifetime is thought to have
     * expired.
     */
    rpcSubscriptions,
  });

  const tokenUri =
    "https://raw.githubusercontent.com/HongThaiPham/summer-bootcamp-anchor-token2022-stake/main/app/assets/token-info.json";

  const collectionMetadata = {
    name: "NFT Collection",
    symbol: "NCT",
    uri: tokenUri,
  };

  const nftMetadata = {
    name: "Minter NFT",
    symbol: "MNT",
    uri: tokenUri,
  };

  const umi = createUmi(`https://${RPC_HOST}`)
    .use(mplBubblegum())
    .use(dasApi());

  umi.use(
    keypairIdentity(
      createSignerFromKeypair(
        umi,
        umi.eddsa.createKeypairFromSecretKey(oldPayer.secretKey)
      )
    )
  );

  return {
    provider,
    connection,
    wallet,
    oldPayer,
    payer,
    payerPublicKey,
    rpc,
    sendAndConfirmTransaction,
    collectionMetadata,
    nftMetadata,
    user2,
    user1,
    tokenUri,
    umi,
  };
}

export const explorerUrl = (tx: string) =>
  `https://explorer.solana.com/tx/${tx}?cluster=devnet`;
