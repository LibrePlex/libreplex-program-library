export type Defaultrenderer = {
  "version": "0.1.0",
  "name": "defaultrenderer",
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
      ]
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

export const IDL: Defaultrenderer = {
  "version": "0.1.0",
  "name": "defaultrenderer",
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
      ]
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
