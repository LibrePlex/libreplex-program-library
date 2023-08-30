import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { LibreplexCreator } from "../target/types/libreplex_creator";
import { LibreplexMetadata } from "../target/types/libreplex_metadata";
import { LibreplexNft } from "../target/types/libreplex_nft";
import { LibreplexCreatorControls } from "../target/types/libreplex_creator_controls";
import { ConfirmOptions, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SYSVAR_EPOCH_SCHEDULE_PUBKEY, SYSVAR_SLOT_HASHES_PUBKEY, Signer, SystemProgram, TransactionInstruction, sendAndConfirmTransaction } from "@solana/web3.js";
import { expect } from 'chai';
import exp from "constants";
import { struct, u8 } from "@solana/buffer-layout";
import { publicKey } from "@solana/buffer-layout-utils";
import {
  MINT_SIZE, TOKEN_2022_PROGRAM_ID, createInitializeMint2Instruction,
  getMinimumBalanceForRentExemptMint, getAssociatedTokenAddressSync, createAssociatedTokenAccountInstruction, createMintToInstruction
} from "@solana/spl-token"
import { Transaction } from "@solana/web3.js";
import {
  LIBREPLEX_METADATA_PROGRAM_ID, setupGroup,
  setUserPermissionsForGroup,
  UserPermission, setupCreator, setupCreatorWithCustomSalePhases, mintFromCreatorController, LIBREPLEX_CREATOR_CONTROLS_PROGRAM_ID
} from "@libreplex/sdk"
import {sha256} from "js-sha256"


describe("libreplex creator", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.LibreplexCreator as Program<LibreplexCreator>;
  const metadataProgram = anchor.workspace.LibreplexMetadata as Program<LibreplexMetadata>;
  const nftProgram = anchor.workspace.LibreplexNft as Program<LibreplexNft>
  const controllerProgram = anchor.workspace.LibreplexCreatorControls as Program<LibreplexCreatorControls>

  console.log(Object.keys(anchor.workspace))

  const authority = anchor.getProvider().publicKey

  it("has minted", async () => {
    const groupSeed = Keypair.generate();

    console.log("Setting up group")
    const grpSetupCtx = await setupGroup({
      connector: {
        type: "provider",
        provider,
      },
      groupSeedKp: groupSeed,
      groupAuthority: program.provider.publicKey as PublicKey,
      input: {
        description: "A very cool group",
        name: "COOL GROUP",
        symbol: "COOL",
        url: "COOL.com/group",
        royalties: {
          bps: 100,
          shares: [{
            recipient: program.provider.publicKey as PublicKey,
            share: 100,
          }],
        }
      }
    })

    const group = grpSetupCtx.group;
    await grpSetupCtx.method.rpc({
      skipPreflight: false,
    });

    const startTime = new Date();
    startTime.setDate(startTime.getDate() - 1)

    const pingDiscrim = Buffer.from(sha256.digest("global:ping")).slice(0, 8)

    console.log("Setting up controller")
    const creatorControllerCtx = await setupCreatorWithCustomSalePhases({
      group,
      metadataProgram,
      mintAuthority: program.provider.publicKey as PublicKey,
      program,
      creatorData: {
        baseName: "COOL #",
        baseUrl: {
          type: "json-prefix",
          url: "COOL.com/",
        },
        description: "The coolest metadatas",
        ordered: false,
        symbol: "COOL",
        supply: 2000
      }
    }, controllerProgram, [{
      end: null,
      start: startTime,
      label: "Public",
      /* No controls anyone can mint and it's free*/
      control: [{
        name: "CustomProgram",
        instructionData: pingDiscrim,
        label: "Ping",
        programId: LIBREPLEX_CREATOR_CONTROLS_PROGRAM_ID,
        remainingAccountsMetas: [{
          isSigner: false,
          isWritable: true,
          key: {
            type: "key",
            value: Keypair.generate().publicKey,
          }
        }, {
          isSigner: false,
          isWritable: true,
          key: {
            type: "seedDerivation",
            programId: SYSVAR_EPOCH_SCHEDULE_PUBKEY,
            seeds: [{
              type: "mintPlaceHolder",
            }],
          }
        }],
      }]
    }])

    await creatorControllerCtx.method.rpc()

    const { creator, minterNumbers, creatorController } = creatorControllerCtx;


    const controllerData = await controllerProgram.account.creatorController.fetch(creatorController)



    console.log("Creator initialised")

    {
      // Set some dummy values for transfer hook.
      const mintMethod = await mintFromCreatorController({
        addTransferHookToMint: {
          authority: program.provider.publicKey as PublicKey,
          programId: program.provider.publicKey as PublicKey,
        },
        creatorController: creatorControllerCtx.creatorController,
        creatorControllerProgram: controllerProgram,
        creatorProgram: program,
      })

      const txId = await mintMethod.method.rpc({
        skipPreflight: true,
      })

      console.log(txId)
    }

    {
      // Mint without transfer hook
      const mintMethod = await mintFromCreatorController({
        creatorController: creatorControllerCtx.creatorController,
        creatorControllerProgram: controllerProgram,
        creatorProgram: program,
      })

      
      await mintMethod.method.rpc({
        skipPreflight: true,
      })
    }

  });


});