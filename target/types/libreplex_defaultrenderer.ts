export type LibreplexDefaultrenderer = {
  "version": "0.1.2",
  "name": "libreplex_defaultrenderer",
  "instructions": [
    {
      "name": "canonical",
      "accounts": [
        {
          "name": "metadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataExtension",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "group",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "renderInput",
          "type": {
            "defined": "RenderInput"
          }
        }
      ],
      "returns": "bytes"
    }
  ],
  "types": [
    {
      "name": "RenderInput",
      "type": {
        "kind": "struct",
        "fields": []
      }
    }
  ]
};

export const IDL: LibreplexDefaultrenderer = {
  "version": "0.1.2",
  "name": "libreplex_defaultrenderer",
  "instructions": [
    {
      "name": "canonical",
      "accounts": [
        {
          "name": "metadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataExtension",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "group",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "renderInput",
          "type": {
            "defined": "RenderInput"
          }
        }
      ],
      "returns": "bytes"
    }
  ],
  "types": [
    {
      "name": "RenderInput",
      "type": {
        "kind": "struct",
        "fields": []
      }
    }
  ]
};
