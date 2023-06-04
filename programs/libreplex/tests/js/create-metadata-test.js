"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const anchor = __importStar(require("@coral-xyz/anchor"));
const web3_js_1 = require("@solana/web3.js");
const chai_1 = require("chai");
describe("libreplex", () => {
    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.Libreplex;
    const authority = anchor.getProvider().publicKey;
    const collectionSeed = web3_js_1.Keypair.generate();
    const collection = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("collection"), collectionSeed.publicKey.toBuffer()], program.programId)[0];
    const userPermissions = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("permissions"), collection.toBuffer(), authority.toBuffer()], program.programId)[0];
    it("has created a collection and metadata", () => __awaiter(void 0, void 0, void 0, function* () {
        const createCollectionEventPromise = new Promise((resolve, reject) => {
            program.addEventListener("CreateCollectionEvent", (event, slot, sig) => {
                resolve(event);
            });
        });
        const permissionEventPromise = new Promise((resolve, reject) => {
            program.addEventListener("PermissionEvent", (event, slot, sig) => {
                resolve(event);
            });
        });
        const createMetadataEventPromise = new Promise((resolve, reject) => {
            program.addEventListener("CreateMetadataEvent", (event, slot, sig) => {
                resolve(event);
            });
        });
        const collectionName = "COOL COLLECTION";
        const tx = yield program.methods.createCollection({
            collectionUrl: "COOL.com",
            name: collectionName,
            symbol: "COOL",
            nftCollectionData: null,
        }).accounts({
            authority,
            seed: collectionSeed.publicKey,
            collection,
            systemProgram: web3_js_1.SystemProgram.programId,
            userPermissions
        }).rpc();
        console.log("Your transaction signature", tx);
        const createCollectionEvent = yield createCollectionEventPromise;
        const permissionEvent = yield permissionEventPromise;
        (0, chai_1.expect)(createCollectionEvent).to.deep.equal({
            creator: authority,
            name: collectionName,
            id: collection,
        });
        (0, chai_1.expect)(permissionEvent).to.deep.equal({
            collection,
            user: authority,
            eventType: {
                update: {},
            }
        });
        console.log("Here");
        const mint = web3_js_1.Keypair.generate();
        const metadata = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("metadata"), mint.publicKey.toBuffer()], program.programId)[0];
        const metadataName = "COOLMETA";
        yield program.methods.createMetadata({
            metadataUrl: "COOLURL.com",
            name: metadataName,
            nftMetadata: null,
        }).accounts({
            mint: mint.publicKey,
            collection,
            metadata,
            systemProgram: web3_js_1.SystemProgram.programId,
            signer: authority,
            signerCollectionPermissions: userPermissions,
        }).signers([mint]).rpc();
        const createMetadataEvent = yield createMetadataEventPromise;
        (0, chai_1.expect)(createMetadataEvent).to.deep.equal({
            id: metadata,
            collection,
            mint: mint.publicKey,
            name: metadataName,
        });
    }));
});
