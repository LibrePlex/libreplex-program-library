import * as anchor from '@coral-xyz/anchor';
import { AnchorProvider, BN, Idl, Program } from '@coral-xyz/anchor';
import { Connection, Keypair, PublicKey, SystemProgram } from '@solana/web3.js';
import { AccountUtils, isKp} from '../prog-common';
import { Libreplex } from '../types/libreplex';


export class LibreplexClient extends AccountUtils {
    wallet: anchor.Wallet;
    provider!: anchor.Provider;
    libreplexProgram!: anchor.Program<Libreplex>;

    constructor(
        conn: Connection,
        wallet: anchor.Wallet,
        idl?: Idl,
        programId?: PublicKey
    ) {
        super(conn);
        this.wallet = wallet;
        this.setProvider();
        this.setLibreplexProgram(idl, programId);
    }

    setProvider() {
        this.provider = new AnchorProvider(
            this.conn,
            this.wallet,
            AnchorProvider.defaultOptions()
        );
        anchor.setProvider(this.provider);
    }

    setLibreplexProgram(idl?: Idl, programId?: PublicKey) {
        //instantiating program depends on the environment
        if (idl && programId) {
            //means running in prod
            this.libreplexProgram = new anchor.Program<Libreplex>(
                idl as any,
                programId,
                this.provider
            );
        } else {
            //means running inside test suite
            this.libreplexProgram = anchor.workspace.BountyPool as Program<Libreplex>;
        }
    }















}
