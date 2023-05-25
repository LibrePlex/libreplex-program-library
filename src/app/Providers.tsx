// app/providers.tsx
"use client";

import { CacheProvider } from "@chakra-ui/next-js";
import { ChakraProvider } from "@chakra-ui/react";
import { useCallback, useMemo, useState } from "react";
import { SolflareWalletAdapter } from "@solana/wallet-adapter-wallets";
import { clusterApiUrl } from "@solana/web3.js";
import {
  ConnectionProvider,
  WalletProvider,
} from "@solana/wallet-adapter-react";
import dynamic from "next/dynamic";
import { WalletError } from "@solana/wallet-adapter-base";

import { useToast } from "@chakra-ui/react";
import NetworkSwitcher from "@/components/networkswitcher/NetworkSwitcher";
import { NetworkConfigurationProvider } from "@/components/networkswitcher/NetworkConfigurationProvider";
import Nav from "@/components/Navbar";

export function Providers({ children }: { children: React.ReactNode }) {
  const network = "devnet";

  const toast = useToast();

  const ReactUIWalletModalProviderDynamic = dynamic(
    async () =>
      (await import("@solana/wallet-adapter-react-ui")).WalletModalProvider,
    { ssr: false }
  );

  const wallets = useMemo(() => [new SolflareWalletAdapter()], [network]);

  const endpoint = useMemo(() => clusterApiUrl(network), [network]);

  const [autoConnect, setAutoConnect]  = useState<boolean>(false);

  const onError = useCallback((error: WalletError) => {
    toast({
      title: "Could not connect wallet.",
      description: error.message
        ? `${error.name}: ${error.message}`
        : error.name,
      status: "error",
      duration: 9000,
      isClosable: true,
    });
    // notify({ type: 'error', message: error.message ? `${error.name}: ${error.message}` : error.name });
    // console.error(error);
  }, []);

  return (
    <CacheProvider>
      <ChakraProvider>
        <NetworkConfigurationProvider>
          <ConnectionProvider endpoint={endpoint}>
          <Nav/>
            <WalletProvider
              wallets={wallets}
              onError={onError}
              autoConnect={autoConnect}
            >
          
              <ReactUIWalletModalProviderDynamic>
                
             
                {children}
              </ReactUIWalletModalProviderDynamic>
            </WalletProvider>
          </ConnectionProvider>
        </NetworkConfigurationProvider>
      </ChakraProvider>
    </CacheProvider>
  );
}
