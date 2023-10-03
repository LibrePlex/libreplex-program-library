import {
  AccountMeta,
  Connection,
  Keypair,
  PublicKey,
  SYSVAR_SLOT_HASHES_PUBKEY,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from '@solana/web3.js';
import { LibreplexCreator } from '@libreplex/idls/lib/types/libreplex_creator';
import { LibreplexMetadata } from '@libreplex/idls/lib/types/libreplex_metadata';
import { LibreplexNft } from '@libreplex/idls/lib/types/libreplex_nft';
import { LibreplexCreatorControls } from '@libreplex/idls/lib/types/libreplex_creator_controls';
import {
  Program,
  AccountClient,
  IdlAccounts,
  IdlTypes,
  Provider,
} from '@coral-xyz/anchor';
import {
  LIBREPLEX_CREATOR_CONTROLS_PROGRAM_ID,
  LIBREPLEX_CREATOR_PROGRAM_ID,
  LIBREPLEX_METADATA_PROGRAM_ID,
  LIBREPLEX_NFT_PROGRAM_ID,
} from './constants';
import {
  MINT_SIZE,
  TOKEN_2022_PROGRAM_ID,
  createInitializeMint2Instruction,
  getMinimumBalanceForRentExemptMint,
  getAssociatedTokenAddressSync,
  createAssociatedTokenAccountInstruction,
  createMintToInstruction,
} from '@solana/spl-token';
import { struct, u8 } from '@solana/buffer-layout';
import { publicKey } from '@solana/buffer-layout-utils';
import { getMetadataAddress, getMintWrapperAddress } from './pda';
import { getCollectionWideUserPermissionsAddress } from './groupPermissions';

import { RoyaltyConfig } from './createCollection';
import { loadMetadataProgram, loadNftProgram } from './programs';

type CustomProgramAccountMeta = Omit<AccountMeta, 'pubkey'> & {
  key:
    | {
        keyType: 'PublicKey';
        value: PublicKey;
      }
    | {
        keyType: 'PDA';
        seeds: Buffer[];
        deriveFromMint: boolean;
        deriveFromBuyer: boolean;
        programIdToDeriveFrom: PublicKey;
      };
};

export type MintFromCreatorControllerInput = {
  creatorControllerProgram: Program<LibreplexCreatorControls>;
  creatorProgram: Program<LibreplexCreator>;
  creatorController: PublicKey;

  mintKeyPair?: Keypair;

  // If there are multiple active sale phases, specify the one to mint in.
  phaseToMintIn?: string;

  merkleProofsForAllowLists?: {
    label: string;
    proof: Buffer[];
  }[];

  addTransferHookToMint?: {
    programId: PublicKey;
    authority: PublicKey;
  };
};

type MintFromCreatorControllerStateInput = {
  creator: PublicKey;
  targetPhase: IdlAccounts<LibreplexCreatorControls>['creatorController']['phases'][0];
  minterNumbers: PublicKey | null;
  collection: PublicKey;
} & Omit<MintFromCreatorControllerInput, 'phaseToMintIn' | 'creatorProgram'>;

export async function mintFromCreatorControllerState(
  input: MintFromCreatorControllerStateInput
) {
  const {
    creatorControllerProgram,
    creatorController,
    merkleProofsForAllowLists,
    addTransferHookToMint,
    minterNumbers,
    targetPhase,
    collection,
    creator,
  } = input;

  let mintKeyPair = input.mintKeyPair || Keypair.generate();

  const connection = creatorControllerProgram.provider.connection;
  const me = creatorControllerProgram.provider.publicKey;

  if (!me) {
    throw new Error('Provider not setup. Perhaps your wallet is not connected');
  }

  const args: Buffer[] = [];
  const remainingAccounts: AccountMeta[] = [];

  const controls = targetPhase.controls;

  for (const control of controls) {
    if (control.payment) {
      remainingAccounts.push({
        isSigner: false,
        isWritable: true,
        pubkey: control.payment[0].recepient,
      });
    } else if (control.splPayment) {
      remainingAccounts.push({
        isSigner: false,
        isWritable: true,
        pubkey: control.splPayment[0].recepient,
      });

      remainingAccounts.push({
        isSigner: false,
        isWritable: true,
        pubkey: getAssociatedTokenAddressSync(
          control.splPayment[0].mint,
          me,
          undefined,
          control.splPayment[0].tokenProgram
        ),
      });

      remainingAccounts.push({
        isSigner: false,
        isWritable: false,
        pubkey: control.splPayment[0].tokenProgram,
      });
    } else if (control.mintLimit) {
      const seeds: Buffer[] = [Buffer.from('mint_limit')];

      if (control.mintLimit[0].scopedToBuyer) {
        seeds.push(me.toBuffer());
      }

      control.mintLimit[0].accountKey.forEach((keyElement) => {
        seeds.push(keyElement.toBuffer());
      });

      const mintLimitAccount = PublicKey.findProgramAddressSync(
        seeds,
        LIBREPLEX_CREATOR_CONTROLS_PROGRAM_ID
      )[0];

      remainingAccounts.push({
        isSigner: false,
        isWritable: true,
        pubkey: mintLimitAccount,
      });
    } else if (control.allowList) {
      if (!merkleProofsForAllowLists) {
        throw new Error(
          'Must provide merkle proofs when your creator as an allowlist'
        );
      }

      const proofEntry = merkleProofsForAllowLists.find(
        (mp) => mp.label === control.allowList[0].label
      );

      if (!proofEntry) {
        throw new Error(
          `Proof entry not found for allowlist: ${control.allowList[0].label}`
        );
      }

      args.push(Buffer.concat(proofEntry.proof));
    } else if (control.customProgram) {
      const remainingAccountMetas: AccountMeta[] = [
        {
          isSigner: false,
          isWritable: false,
          pubkey: control.customProgram[0].programId,
        },
      ];

      for (const meta of control.customProgram[0].remainingAccountMetas) {
        const key: IdlTypes<LibreplexCreatorControls>['CustomProgramAcountMetaKey'] =
          meta.key as any;

        if (key.pubkey) {
          remainingAccountMetas.push({
            ...meta,
            pubkey: key.pubkey[0],
          });
        } else if (key.derivedFromSeeds) {
          const programId = key.derivedFromSeeds[0].programId;

          const seeds: Buffer[] = [];
          for (const seed of key.derivedFromSeeds[0].seeds) {
            if (seed.bytes) {
              seeds.push(seed.bytes[0]);
            } else if (seed.mintPlaceHolder) {
              seeds.push(mintKeyPair.publicKey.toBuffer());
            } else if (seed.payerPlaceHolder || seed.receiverPlaceHolder) {
              seeds.push(me.toBuffer());
            } else {
              throw new Error('Invalid seed derivation');
            }
          }

          remainingAccountMetas.push({
            ...meta,
            pubkey: PublicKey.findProgramAddressSync(seeds, programId)[0],
          });
        } else {
          throw new Error('Invalid CustomProgramAcountMetaKey');
        }
      }

      remainingAccounts.push(...remainingAccountMetas);
    }
  }

  const metadata = getMetadataAddress(mintKeyPair.publicKey);
  const setupMintCtx = await setupLibreplexReadyMint(
    connection,
    me,
    me,
    me,
    me,
    0,
    mintKeyPair,
    metadata,
    addTransferHookToMint
  );

  return {
    method: creatorControllerProgram.methods
      .mint({
        chosenPhase: targetPhase.label,
        args,
      })
      .accounts({
        attributeConfig: null,
        creator,
        creatorController,
        collection,
        libreplexCreatorProgram: LIBREPLEX_CREATOR_PROGRAM_ID,
        libreplexMetadataProgram: LIBREPLEX_METADATA_PROGRAM_ID,
        libreplexNftProgram: LIBREPLEX_NFT_PROGRAM_ID,
        mint: mintKeyPair.publicKey,
        metadata,
        mintAuthority: me,
        minterNumbers,
        mintWrapper: getMintWrapperAddress(mintKeyPair.publicKey),
        payer: me,
        receiver: me,
        receiverTokenAccount: getAssociatedTokenAddressSync(
          mintKeyPair.publicKey,
          me,
          undefined,
          TOKEN_2022_PROGRAM_ID
        ),
        recentSlothashes: SYSVAR_SLOT_HASHES_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        collectionPermissions: getCollectionWideUserPermissionsAddress(
          collection,
          creator
        ),
      })
      .preInstructions([...setupMintCtx.transaction.instructions])
      .signers([mintKeyPair])
      .remainingAccounts(remainingAccounts),

    mint: mintKeyPair,
  };
}

export async function mintFromCreatorController(
  input: MintFromCreatorControllerInput
) {
  const {
    creatorControllerProgram,
    creatorController,
    creatorProgram,
    phaseToMintIn,
  } = input;

  const controller =
    await creatorControllerProgram.account.creatorController.fetchNullable(
      creatorController
    );

  if (!controller) {
    throw new Error(
      `Creator controller at address: ${creatorController.toString()} not found`
    );
  }

  const creator = await creatorProgram.account.creator.fetchNullable(
    controller.creator
  );

  if (!creator) {
    throw new Error(
      `Creator at address ${controller.creator?.toString()} not found`
    );
  }

  const now = Date.now() / 1000;

  const availableSalePhases = controller.phases;

  const activePhases = availableSalePhases.filter(
    (ph) => now > ph.start && (ph.end === null || now < ph.end)
  );

  if (activePhases.length === 0) {
    throw new Error('No currently active phases to mint from');
  }

  let targetPhase = activePhases[0];

  if (activePhases.length > 1) {
    if (!phaseToMintIn) {
      throw new Error(
        'Must provide a target phase to mint in when multiple are active'
      );
    }

    const maybeTargetPhase = activePhases.find(
      (ph) => ph.label === phaseToMintIn
    );

    if (!maybeTargetPhase) {
      throw new Error(
        `Specified phase to mint in ${phaseToMintIn} is not active`
      );
    }

    targetPhase = maybeTargetPhase;
  }

  return mintFromCreatorControllerState({
    ...input,
    targetPhase,
    creator: controller.creator,
    minterNumbers: creator.minterNumbers,
    collection: creator.collection,
  });
}

export type MintAssetUrl =
  | {
      type: 'jsonUrl';
      value: string;
    }
  | {
      type: 'imageUrl';
      value: string;
    }
  | {
      type: 'renderedOnChain';
      programId: PublicKey;
      description: string | null;
    };

export type MetadataExtension = {
  // Metadata extension data. If you don't want it don't take it.

  licenseUrl?: string;

  /**
   * The list of keys that can add their signature to your metadata.
   */
  permittedSigners?: PublicKey[];

  /**
   * Only works when part of a group.
   * Attribute defintions exist on the group.
   * List of pointers to the on chain attributes stored in the group.
   */
  onChainAttributes?: number[];

  royalties?: RoyaltyConfig;
};

export type MetadataData = {
  name: string;
  symbol: string;
  assetUrl: MintAssetUrl;

  extension?: MetadataExtension;
};

export type MintSingleInput = {
  provider: Provider;

  mintData: MetadataData;

  mintToCollection?: {
    collection: PublicKey;
    checkValidGroup: boolean;

    /**
     * If you are not the update auth of the group.
     * But have been given permission to add metadatas to it.
     * Set this to true.
     *
     * Defaults to false.
     */
    groupDelegate?: boolean;
  };

  mintKp?: Keypair;

  receiver?: PublicKey;

  transferHook?: TransferHookConfig;

  metadataProgram?: Program<LibreplexMetadata>;
  nftProgram?: Program<LibreplexNft>;

  updateAuthority?: PublicKey;
};

export async function mintSingle(input: MintSingleInput) {
  const {
    provider,
    metadataProgram = await loadMetadataProgram(provider),
    nftProgram = await loadNftProgram(provider),
    mintToCollection,
    receiver = provider.publicKey,
    mintKp = Keypair.generate(),
    transferHook,
    updateAuthority = provider.publicKey,
    mintData,
  } = input;

  const me = provider.publicKey;

  if (!me) {
    throw new Error(
      'Provider does have a wallet loaded into it. Are you sure your wallet is connected'
    );
  }

  if (mintToCollection) {
    if (mintToCollection.checkValidGroup) {
      const groupData = await metadataProgram.account.collection.fetchNullable(
        mintToCollection.collection
      );

      if (!groupData) {
        throw new Error('Group does not exist');
      }

      if (groupData.updateAuthority.toString() != me.toString()) {
        const groupWideAddress = getCollectionWideUserPermissionsAddress(
          mintToCollection.collection,
          me
        );

        const permissionsData =
          await metadataProgram.account.delegatePermissions.fetchNullable(
            groupWideAddress
          );

        const hasDelegatedPermission = !!permissionsData?.permissions.find(
          (perm) => !!perm.addToGroup
        );

        if (!permissionsData || !hasDelegatedPermission) {
          throw new Error(
            'You do not have permission to add metadata to this group.'
          );
        }

        mintToCollection.groupDelegate = true;
      }
    }
  }

  const connection = provider.connection;

  const metadata = getMetadataAddress(mintKp.publicKey);

  const mintCtx = await setupLibreplexReadyMint(
    connection,
    me,
    me,
    receiver as PublicKey,
    me,
    0,
    mintKp,
    metadata,
    transferHook
  );

  let anchorAssetUrl: IdlTypes<LibreplexMetadata>['Asset'];

  const { assetUrl, name, symbol } = mintData;

  switch (assetUrl.type) {
    case 'jsonUrl':
      anchorAssetUrl = {
        json: {
          url: assetUrl.value,
        },
      };
      break;
    case 'imageUrl':
      anchorAssetUrl = {
        image: {
          url: assetUrl.value,
          description: null,
        },
      };
      break;
    case 'renderedOnChain':
      anchorAssetUrl = {
        chainRenderer: {
          programId: assetUrl.programId,
        },
      };
      break;

    default:
      throw new Error('Invalid asset type');
  }

  type ExtensionType = IdlTypes<LibreplexMetadata>['MetadataExtension'];
  let extensions: ExtensionType[] = [];

  if (mintData.extension) {
    if (mintData.extension.onChainAttributes) {
      extensions.push({
        attributes: {
          attributes: Buffer.from(mintData.extension.onChainAttributes ?? []),
        },
      });
    }
    if (mintData.extension.royalties) {
      extensions.push({
        royalties: {
          royalties: mintData.extension.royalties || null,
        },
      });
    }

    if (mintData.extension.licenseUrl) {
      extensions.push({
        license: {
          license: {
            custom: {
              licenseUrl: mintData.extension.licenseUrl,
            },
          },
        },
      });
    }

    if (mintData.extension.permittedSigners) {
      extensions.push({
        signers: {
          signers: mintData.extension.permittedSigners || [],
        },
      });
    }
  }
  console.log('Creating metadata instructions');
  const createMetaData = metadataProgram.methods
    .createMetadata({
      asset: anchorAssetUrl,
      extensions,
      name,
      symbol,
      updateAuthority: updateAuthority as PublicKey,
    })
    .accounts({
      authority: me,
      metadata,
      payer: me,
      mint: mintKp.publicKey,
      invokedMigratorProgram: null,
      systemProgram: SystemProgram.programId,
    })
    .preInstructions(mintCtx.transaction.instructions)
    .signers([mintCtx.keypair]);

  console.log('Created metadata instruction');

  const postIxs: TransactionInstruction[] = [
    await nftProgram.methods
      .wrap()
      .accounts({
        authority: me,
        mint: mintKp.publicKey,
        payer: me,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        wrappedMint: getMintWrapperAddress(mintKp.publicKey),
      })
      .instruction(),
  ];

  if (mintToCollection) {
    const ix = await metadataProgram.methods
      .addMetadataToCollection()
      .accounts({
        delegatedCollectionWidePermissions: mintToCollection.groupDelegate
          ? getCollectionWideUserPermissionsAddress(
              mintToCollection.collection,
              me
            )
          : null,
        systemProgram: SystemProgram.programId,
        payer: me,
        metadata,
        metadataAuthority: me,
        collectionAuthority: me,
        delegatedMetadataSpecificPermissions: null,
        collection: mintToCollection.collection,
      })
      .instruction();

    postIxs.push(ix);
  }

  return {
    method: createMetaData.postInstructions(postIxs),
    mint: mintKp,
  };
}

const MetadataPointerMintSize = 234;
const MintSizeForTranserHookAndPointer = 302;

interface InitializeMetadataPointerIx {
  instruction: 39;
  metadataPointerInitIx: 0;
  authority: PublicKey;
  metadataAddress: PublicKey;
}

const initializeMetadataPointerInstructionData =
  struct<InitializeMetadataPointerIx>([
    u8('instruction') as any,
    u8('metadataPointerInitIx'),
    publicKey('authority'),
    publicKey('metadataAddress'),
  ]);

interface InitializeTransferHookInit {
  instruction: 36;
  transferHookInstruction: 0;
  authority: PublicKey;
  transferHookProgramId: PublicKey;
}

const initializeTransferHookInitInstructionData =
  struct<InitializeTransferHookInit>([
    u8('instruction') as any,
    u8('transferHookInstruction'),
    publicKey('authority'),
    publicKey('transferHookProgramId'),
  ]);

type TransferHookConfig = {
  programId: PublicKey;
  authority: PublicKey;
};

export async function setupLibreplexReadyMint(
  connection: Connection,
  payer: PublicKey,
  receiver: PublicKey,
  mintAuthority: PublicKey,
  freezeAuthority: PublicKey | null,
  decimals: number,
  mintKeypair = Keypair.generate(),
  metadata: PublicKey,
  transferHook?: TransferHookConfig,
  programId = TOKEN_2022_PROGRAM_ID
) {
  const mintSize = transferHook
    ? MintSizeForTranserHookAndPointer
    : MetadataPointerMintSize;
  const lamports = await connection.getMinimumBalanceForRentExemption(mintSize);

  const initMetadataPointerExtensionIx = (() => {
    const initMetadataPointerIxSpan = Buffer.alloc(
      initializeMetadataPointerInstructionData.span
    );

    initializeMetadataPointerInstructionData.encode(
      {
        instruction: 39,
        authority: PublicKey.default,
        metadataPointerInitIx: 0,
        metadataAddress: metadata,
      },
      initMetadataPointerIxSpan
    );

    return new TransactionInstruction({
      keys: [
        {
          isSigner: false,
          isWritable: true,
          pubkey: mintKeypair.publicKey,
        },
      ],
      programId,
      data: initMetadataPointerIxSpan,
    });
  })();

  const preInitMintIxs: TransactionInstruction[] = [];

  if (transferHook) {
    const accounts = [
      { pubkey: mintKeypair.publicKey, isSigner: false, isWritable: true },
    ];
    const transferHookIxBuf = Buffer.alloc(
      initializeTransferHookInitInstructionData.span
    );
    initializeTransferHookInitInstructionData.encode(
      {
        authority: transferHook.authority,
        transferHookProgramId: transferHook.programId,
        instruction: 36,
        transferHookInstruction: 0,
      },
      transferHookIxBuf
    );

    preInitMintIxs.push(
      new TransactionInstruction({
        keys: accounts,
        programId: TOKEN_2022_PROGRAM_ID,
        data: transferHookIxBuf,
      })
    );
  }

  const assocTokenAccount = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    receiver,
    undefined,
    TOKEN_2022_PROGRAM_ID
  );
  const transaction = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payer,
      newAccountPubkey: mintKeypair.publicKey,
      space: mintSize,
      lamports,
      programId,
    }),
    initMetadataPointerExtensionIx,
    ...preInitMintIxs,
    createInitializeMint2Instruction(
      mintKeypair.publicKey,
      decimals,
      mintAuthority,
      freezeAuthority,
      programId
    ),
    createAssociatedTokenAccountInstruction(
      payer,
      assocTokenAccount,
      receiver,
      mintKeypair.publicKey,
      TOKEN_2022_PROGRAM_ID
    ),
    createMintToInstruction(
      mintKeypair.publicKey,
      assocTokenAccount,
      mintAuthority,
      1,
      undefined,
      TOKEN_2022_PROGRAM_ID
    )
  );

  return {
    transaction,
    keypair: mintKeypair,
  };
}
