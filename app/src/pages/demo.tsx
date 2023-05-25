"use client";

import { Box, Heading, ListItem, Text, UnorderedList } from "@chakra-ui/react";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { SendTransaction } from "components/SendTransaction";
import { SendVersionedTransaction } from "components/SendVersionedTransaction";
import { SignMessage } from "components/SignMessage";
import { PROGRAM_ID as LIBREPLEX_PROGRAM_ID } from "@libreplex/js";
const Demo = () => {
  // Here is a wallet adapter
  const wallet = useWallet();

  // Here is an RPC connection
  const connection = useConnection();

  // The Solita client can be imported via yarn workspace
  const id = LIBREPLEX_PROGRAM_ID;

  return (
    <Box
      w={"100vw"}
      h={"100vh"}
      display={"flex"}
      flexDirection={"column"}
      sx={{
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <Box
        sx={{
          maxWidth: "600px",
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
        }}
      >
        <Heading variant={"h2"} sx={{ maxWidth: "500px", pb: 3 }}>
          Demo
        </Heading>

        <div className="text-center">
          <SignMessage />
          <SendTransaction />
          <SendVersionedTransaction />
        </div>

        <Text>Functionality</Text>

        <UnorderedList>
          <ListItem>Create NFT Collection</ListItem>
          <ListItem>Delete NFT Collection</ListItem>
          <ListItem>Create SPL Collection</ListItem>
          <ListItem>Delete SPL Collection</ListItem>
          <ListItem>Create NFT Metadata</ListItem>
          <ListItem>Delete NFT Metadata</ListItem>
          <ListItem>Create SPL Metadata</ListItem>
          <ListItem>Delete SPL Metadata</ListItem>
          <ListItem>Verify (sign) creator</ListItem>
          <ListItem>TBD: Unverify creator</ListItem>
        </UnorderedList>

        <UnorderedList>
          <ListItem>Delegation</ListItem>
          <ListItem>etc etc</ListItem>
        </UnorderedList>
      </Box>
    </Box>
  );
};

export default Demo;
