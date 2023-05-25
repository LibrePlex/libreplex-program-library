import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import {
  AccountInfo,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  MintInfo,
  Token,
  TOKEN_PROGRAM_ID,
} from '@solana/spl-token';

export interface ITokenData {
  tokenMint: PublicKey;
  tokenAcc: PublicKey;
  owner: PublicKey;
  token: Token;
}

export class AccountUtils {
  conn: Connection;

  constructor(connection: Connection) {
    this.conn = connection;
  }

  // --------------------------------------- Find PDA's

  async findProgramAddress(
      seeds: (PublicKey | Uint8Array | string)[],
      programId: PublicKey,
  ): Promise<[PublicKey, number]> {
    const seed_bytes = seeds.map((s) => {
      if (typeof s == 'string') {
        return Buffer.from(s);
      } else if ('toBytes' in s) {
        return s.toBytes();
      } else {
        return s;
      }
    });
    return await PublicKey.findProgramAddress(seed_bytes, programId);
  }

  // --------------------------------------- Get Solana Balances - Program Accounts

  async getBalance(publicKey: PublicKey): Promise<number> {
    return this.conn.getBalance(publicKey);
  }

  // --------------------------------------- Get SPL-Token Account Information

  async deserializeToken(mint: PublicKey): Promise<Token> {
    //doesn't matter which keypair goes here, we just need some key for instantiation
    const throwawayKeypair = Keypair.fromSecretKey(
        Uint8Array.from([234,131,46,166,162,188,1,232,35,185,11,83,47,97,247,228,86,118,
                         3,25,241,122,197,163,240,56,225,30,199,245,155,40,196,26,132,41,
                         108,216,12,202,28,21,5,118,125,157,206,248,98,219,192,71,254,42,
                         211,255,55,191,129,243,151,55,90,221])
    );
    return new Token(this.conn, mint, TOKEN_PROGRAM_ID, throwawayKeypair);
  }

  async deserializeTokenMint(mint: PublicKey): Promise<MintInfo> {
    const t = await this.deserializeToken(mint);
    return t.getMintInfo();
  }

  async deserializeTokenAccount(
    mint: PublicKey,
    tokenAccount: PublicKey
  ): Promise<AccountInfo> {
    const token = await this.deserializeToken(mint);
    return token.getAccountInfo(tokenAccount);
  }

  async findATA(mint: PublicKey, owner: PublicKey): Promise<PublicKey> {
    return Token.getAssociatedTokenAddress(
      ASSOCIATED_TOKEN_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      mint,
      owner
    );
  }
}
