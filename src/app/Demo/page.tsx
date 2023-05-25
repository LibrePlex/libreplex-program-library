"use client";

import { Box, Heading, ListItem, Text, UnorderedList } from "@chakra-ui/react";
import PageLayout from "./PageLayout";
import { NextPage } from "next";
import { NextPageWithLayout } from "@/components/common/NextPageWithLayout";
export const Mission: NextPageWithLayout = () => {
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

export default Mission;

Mission.getLayout = function getLayout(page: any) {
  return <PageLayout>{page}</PageLayout>;
};
