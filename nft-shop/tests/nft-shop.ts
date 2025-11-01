import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftShop } from "../target/types/nft_shop";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";

// Metaplex Token Metadata Program ID
const METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
import { assert } from "chai";

describe("nft-shop", () => {
  // TODO : use surfpool or mainnet-beta
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.NftShop as Program<NftShop>;
  const wallet = provider.wallet as anchor.Wallet;

  // PDAs
  let mintAuthorityPda: anchor.web3.PublicKey;
  let mintCountPda: anchor.web3.PublicKey;

  // Collection mint
  let collectionMint: anchor.web3.Keypair;
  let collectionMetadata: anchor.web3.PublicKey;
  let collectionMasterEdition: anchor.web3.PublicKey;
  let collectionTokenAccount: anchor.web3.PublicKey;

  // Track initial counter value
  let initialCounterValue: number;

  before(async () => {
    // Derive PDAs
    [mintAuthorityPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("authority")],
      program.programId
    );

    [mintCountPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("mint_count")],
      program.programId
    );

    // Generate collection mint keypair
    collectionMint = anchor.web3.Keypair.generate();

    // Derive metadata and master edition PDAs
    [collectionMetadata] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        METADATA_PROGRAM_ID.toBuffer(),
        collectionMint.publicKey.toBuffer(),
      ],
      METADATA_PROGRAM_ID
    );

    [collectionMasterEdition] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        METADATA_PROGRAM_ID.toBuffer(),
        collectionMint.publicKey.toBuffer(),
        Buffer.from("edition"),
      ],
      METADATA_PROGRAM_ID
    );

    // Get collection token account
    collectionTokenAccount = await getAssociatedTokenAddress(
      collectionMint.publicKey,
      wallet.publicKey
    );
  });

  describe("Initialization", () => {
    it("Initializes the NFT collection shop", async () => {
      const tx = await program.methods
        .initShop(
          "T3 Collection",
          "T3C",
          "https://turbin3.org"
        )
        .accounts({
          user: wallet.publicKey,
          mint: collectionMint.publicKey,
          mintAuthority: mintAuthorityPda,
          metadata: collectionMetadata,
          masterEdition: collectionMasterEdition,
          destination: collectionTokenAccount,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenMetadataProgram: METADATA_PROGRAM_ID,
        })
        .signers([collectionMint])
        .rpc();

      console.log("Collection initialized:", tx);

      // Verify collection mint was created
      const mintInfo = await provider.connection.getAccountInfo(
        collectionMint.publicKey
      );
      assert.isNotNull(mintInfo, "Collection mint should exist");

      // Verify user received the collection NFT
      const tokenAccount = await provider.connection.getTokenAccountBalance(
        collectionTokenAccount
      );
      assert.equal(
        tokenAccount.value.uiAmount,
        1,
        "User should have 1 collection NFT"
      );
    });

    it("Initializes the mint counter", async () => {
      const tx = await program.methods
        .initMintCount()
        .accounts({
          payer: wallet.publicKey,
          mintCount: mintCountPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      console.log("Mint counter initialized:", tx);

      // Verify mint counter account was created
      const mintCountAccount = await program.account.mintCount.fetch(
        mintCountPda
      );

      // Store the initial value for relative tracking in later tests
      initialCounterValue = mintCountAccount.mintCount.toNumber();

      assert.equal(
        initialCounterValue,
        0,
        "Counter should start at 0"
      );
    });
  });

  describe("NFT Minting", () => {
    it("Mints the first NFT and increments counter", async () => {
      // Generate mint for first NFT
      const nftMint = anchor.web3.Keypair.generate();

      // Derive NFT metadata and master edition
      const [nftMetadata] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          METADATA_PROGRAM_ID.toBuffer(),
          nftMint.publicKey.toBuffer(),
        ],
        METADATA_PROGRAM_ID
      );

      const [nftMasterEdition] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          METADATA_PROGRAM_ID.toBuffer(),
          nftMint.publicKey.toBuffer(),
          Buffer.from("edition"),
        ],
        METADATA_PROGRAM_ID
      );

      const nftTokenAccount = await getAssociatedTokenAddress(
        nftMint.publicKey,
        wallet.publicKey
      );

      // Mint NFT
      const tx = await program.methods
        .mintNft()
        .accounts({
          owner: wallet.publicKey,
          mint: nftMint.publicKey,
          destination: nftTokenAccount,
          metadata: nftMetadata,
          masterEdition: nftMasterEdition,
          mintAuthority: mintAuthorityPda,
          collectionMint: collectionMint.publicKey,
          mintCount: mintCountPda,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenMetadataProgram: METADATA_PROGRAM_ID,
        })
        .signers([nftMint])
        .rpc();

      console.log("First NFT minted:", tx);

      // Verify counter incremented
      const mintCountAccount = await program.account.mintCount.fetch(
        mintCountPda
      );
      const currentCount = mintCountAccount.mintCount.toNumber();
      assert.equal(
        currentCount,
        initialCounterValue + 1,
        `Counter should increment by 1 after first NFT `
      );

      // Verify NFT was minted
      const tokenAccount = await provider.connection.getTokenAccountBalance(
        nftTokenAccount
      );
      assert.equal(
        tokenAccount.value.uiAmount,
        1,
        "User should have 1 NFT"
      );
    });

    it("Mints a second NFT and increments counter to 1", async () => {
      // Generate mint for second NFT
      const nftMint = anchor.web3.Keypair.generate();

      const [nftMetadata] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          METADATA_PROGRAM_ID.toBuffer(),
          nftMint.publicKey.toBuffer(),
        ],
        METADATA_PROGRAM_ID
      );

      const [nftMasterEdition] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          METADATA_PROGRAM_ID.toBuffer(),
          nftMint.publicKey.toBuffer(),
          Buffer.from("edition"),
        ],
        METADATA_PROGRAM_ID
      );

      const nftTokenAccount = await getAssociatedTokenAddress(
        nftMint.publicKey,
        wallet.publicKey
      );

      const tx = await program.methods
        .mintNft()
        .accounts({
          owner: wallet.publicKey,
          mint: nftMint.publicKey,
          destination: nftTokenAccount,
          metadata: nftMetadata,
          masterEdition: nftMasterEdition,
          mintAuthority: mintAuthorityPda,
          collectionMint: collectionMint.publicKey,
          mintCount: mintCountPda,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenMetadataProgram: METADATA_PROGRAM_ID,
        })
        .signers([nftMint])
        .rpc();

      console.log("Second NFT minted:", tx);

      const mintCountAccount = await program.account.mintCount.fetch(
        mintCountPda
      );
      const currentCount = mintCountAccount.mintCount.toNumber();
      assert.equal(
        currentCount,
        initialCounterValue + 2,
        `Counter should increment by 2 after`
      );
    });

    it("Mints a third NFT and increments counter to 3", async () => {
      const nftMint = anchor.web3.Keypair.generate();

      const [nftMetadata] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          METADATA_PROGRAM_ID.toBuffer(),
          nftMint.publicKey.toBuffer(),
        ],
        METADATA_PROGRAM_ID
      );

      const [nftMasterEdition] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          METADATA_PROGRAM_ID.toBuffer(),
          nftMint.publicKey.toBuffer(),
          Buffer.from("edition"),
        ],
        METADATA_PROGRAM_ID
      );

      const nftTokenAccount = await getAssociatedTokenAddress(
        nftMint.publicKey,
        wallet.publicKey
      );

      const tx = await program.methods
        .mintNft()
        .accounts({
          owner: wallet.publicKey,
          mint: nftMint.publicKey,
          destination: nftTokenAccount,
          metadata: nftMetadata,
          masterEdition: nftMasterEdition,
          mintAuthority: mintAuthorityPda,
          collectionMint: collectionMint.publicKey,
          mintCount: mintCountPda,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenMetadataProgram: METADATA_PROGRAM_ID,
        })
        .signers([nftMint])
        .rpc();

      console.log("Third NFT minted:", tx);

      // Verify final counter state
      const mintCountAccount = await program.account.mintCount.fetch(
        mintCountPda
      );
      const currentCount = mintCountAccount.mintCount.toNumber();
      assert.equal(
        currentCount,
        initialCounterValue + 3,
        `Counter should increment by 3 after`
      );
    });
  });

  describe("Counter Verification", () => {
    it("Verifies the mint counter is accessible and accurate", async () => {
      const mintCountAccount = await program.account.mintCount.fetch(
        mintCountPda
      );

      const finalCount = mintCountAccount.mintCount.toNumber();
      assert.equal(
        finalCount,
        initialCounterValue + 3,
        `Counter should have incremented by 6 total`
      );
    });

    it("Verifies counter account has correct owner", async () => {
      const accountInfo = await provider.connection.getAccountInfo(mintCountPda);

      assert.isNotNull(accountInfo, "Counter account should exist");
      assert.equal(
        accountInfo.owner.toString(),
        program.programId.toString(),
        "Counter should be owned by the program"
      );
    });
  });
});
