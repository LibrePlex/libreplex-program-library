type NetworkConfig = {
    clusterApiUrl: string,
    signerKeypair: string
}

// Authority account keypair
export const networkConfig: NetworkConfig =
    {
        clusterApiUrl: "https://api.devnet.solana.com",
        signerKeypair: "/home/charalambos/.config/solana/devnet-libreplex/authority.json"
    }
