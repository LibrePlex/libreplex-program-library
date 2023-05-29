module.exports = {
    validator: {
         relay: {
    enabled: process.env.CI == null,
    killlRunningRelay: true,
  },
        killRunningValidators: true,
         websocketUrl: '',
        commitment: 'confirmed',
        resetLedger: true,
        verifyFees: false,
        detached: process.env.CI != null,
        // By default Amman will pull the account data from the accountsCluster (can be overridden on a per account basis)
        accountsCluster: 'https://api.metaplex.solana.com',
        accounts: [
            {
                label: 'Token Metadata Program',
                accountId: 'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s',
                // marking executable as true will cause Amman to pull the executable data account as well automatically
                executable: true,
            },

            {
                label: 'Token Program',
                accountId: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
                // marking executable as true will cause Amman to pull the executable data account as well automatically
                executable: false,
            },

            {
                label: 'AtaProgram',
                accountId: 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL',
                // marking executable as true will cause Amman to pull the executable data account as well automatically
                executable: false
            },
            {
                label: 'Address lookup table',
                accountId: 'AddressLookupTab1e1111111111111111111111111',
                // marking executable as true will cause Amman to pull the executable data account as well automatically
                executable: false
            },
            //{
            //  label: 'Random other account',
            //  accountId:'4VLgNs1jXgdciSidxcaLKfrR9WjATkj6vmTm5yCwNwui',
            // By default executable is false and is not required to be in the config
            // executable: false,

            // Providing a cluster here will override the accountsCluster field
            //  cluster: 'https://metaplex.devnet.rpcpool.com'
            // }
        ]
    }
}
