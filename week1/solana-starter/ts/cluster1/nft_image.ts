import wallet from "../wba-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { readFileSync } from "fs";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
  try {
    //1. Load image
    const image = readFileSync("turbin3-rug.webp");
    //2. Convert image to generic file.
    const genericFile = createGenericFile(image, "turbin3rug.webp");
    //3. Upload image
    const imageUri = await umi.uploader.upload([genericFile]);
    console.log("Your image URI: ", imageUri);
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }
})();
