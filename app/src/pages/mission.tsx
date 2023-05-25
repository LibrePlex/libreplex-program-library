"use client";

import { Box, Heading, ListItem, Text, UnorderedList } from "@chakra-ui/react";
export const Mission = () => {
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
          Mission
        </Heading>
        <Text>
          The mission of Libreplex is to provide a community-driven, open
          license protocol to the Solana SPL Token and NFT community. The
          protocol must meet the following criteria
        </Text>

        <UnorderedList>
          <ListItem>
            Distributed deployment keys To ensure that no single entity can
            unilaterally make changes that impact or jeopardise the integrity of
            the applications that depend on the protocol
          </ListItem>
          <ListItem>
            Open license held by a Trust The licensing must ensure that any
            applications utilising the protocol can do so knowing that the
            nature of the protocol remains constant, to minimise uncertainty and
            maximise transparency.
          </ListItem>
          <ListItem>
            Guaranteed fees-free for life The fee-free nature of the protocol
            ensures that even though applications built on top of the protocol
            may introduce fees, the protocol itself will never do so. This
            establishes a level playing field to all and enforces predictability
            and transparency.
          </ListItem>
          <ListItem>
            Open source The source of the protocol will be made available on
            github or similar. After initial launch, any changes will be subject
            to 30-day vetting and a community vote.
          </ListItem>
        </UnorderedList>
      </Box>
    </Box>
  );
};

export default Mission;
