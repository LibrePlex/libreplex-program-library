export * from './Attribute'
export * from './Collection'
export * from './CollectionNftData'
export * from './Metadata'
export * from './MetadataNft'
export * from './MetadataNftOverride'
export * from './RoyaltyShare'
export * from './Verification'

import { RoyaltyShare } from './RoyaltyShare'
import { Verification } from './Verification'
import { Attribute } from './Attribute'
import { Metadata } from './Metadata'
import { MetadataNft } from './MetadataNft'
import { MetadataNftOverride } from './MetadataNftOverride'
import { Collection } from './Collection'
import { CollectionNftData } from './CollectionNftData'

export const accountProviders = {
  RoyaltyShare,
  Verification,
  Attribute,
  Metadata,
  MetadataNft,
  MetadataNftOverride,
  Collection,
  CollectionNftData,
}
