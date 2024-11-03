import wallet from "./wallet/Turbin3-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { readFile } from "fs/promises";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
  try {
    //1. Load image
    //2. Convert image to generic file.
    //3. Upload image to arweave
    const image = await readFile(
      "/home/aylen/Development/Solana Turbin3/solana-starter/ts/cluster1/generug.png"
    );
    const myUri = createGenericFile(image, "rug", { contentType: "image/png" });
    const uri = await umi.uploader.upload([myUri]);

    console.log("Your image URI: ", uri);
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }
})();
