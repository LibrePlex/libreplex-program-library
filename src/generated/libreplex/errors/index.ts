/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

type ErrorWithCode = Error & { code: number }
type MaybeErrorWithCode = ErrorWithCode | null | undefined

const createErrorFromCodeLookup: Map<number, () => ErrorWithCode> = new Map()
const createErrorFromNameLookup: Map<string, () => ErrorWithCode> = new Map()

/**
 * InvalidBump: 'Bad bump'
 *
 * @category Errors
 * @category generated
 */
export class InvalidBumpError extends Error {
  readonly code: number = 0x1770
  readonly name: string = 'InvalidBump'
  constructor() {
    super('Bad bump')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidBumpError)
    }
  }
}

createErrorFromCodeLookup.set(0x1770, () => new InvalidBumpError())
createErrorFromNameLookup.set('InvalidBump', () => new InvalidBumpError())

/**
 * MissingBump: 'Missing bump'
 *
 * @category Errors
 * @category generated
 */
export class MissingBumpError extends Error {
  readonly code: number = 0x1771
  readonly name: string = 'MissingBump'
  constructor() {
    super('Missing bump')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, MissingBumpError)
    }
  }
}

createErrorFromCodeLookup.set(0x1771, () => new MissingBumpError())
createErrorFromNameLookup.set('MissingBump', () => new MissingBumpError())

/**
 * CannotRemoveVerifiedCreator: 'Cannot remove verified creator'
 *
 * @category Errors
 * @category generated
 */
export class CannotRemoveVerifiedCreatorError extends Error {
  readonly code: number = 0x1772
  readonly name: string = 'CannotRemoveVerifiedCreator'
  constructor() {
    super('Cannot remove verified creator')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, CannotRemoveVerifiedCreatorError)
    }
  }
}

createErrorFromCodeLookup.set(
  0x1772,
  () => new CannotRemoveVerifiedCreatorError()
)
createErrorFromNameLookup.set(
  'CannotRemoveVerifiedCreator',
  () => new CannotRemoveVerifiedCreatorError()
)

/**
 * CannotAddVerifiedCreator: 'Cannot add verified creator'
 *
 * @category Errors
 * @category generated
 */
export class CannotAddVerifiedCreatorError extends Error {
  readonly code: number = 0x1773
  readonly name: string = 'CannotAddVerifiedCreator'
  constructor() {
    super('Cannot add verified creator')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, CannotAddVerifiedCreatorError)
    }
  }
}

createErrorFromCodeLookup.set(0x1773, () => new CannotAddVerifiedCreatorError())
createErrorFromNameLookup.set(
  'CannotAddVerifiedCreator',
  () => new CannotAddVerifiedCreatorError()
)

/**
 * Attempts to resolve a custom program error from the provided error code.
 * @category Errors
 * @category generated
 */
export function errorFromCode(code: number): MaybeErrorWithCode {
  const createError = createErrorFromCodeLookup.get(code)
  return createError != null ? createError() : null
}

/**
 * Attempts to resolve a custom program error from the provided error name, i.e. 'Unauthorized'.
 * @category Errors
 * @category generated
 */
export function errorFromName(name: string): MaybeErrorWithCode {
  const createError = createErrorFromNameLookup.get(name)
  return createError != null ? createError() : null
}