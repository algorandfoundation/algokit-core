import { Buffer } from 'node:buffer'
import { randomBytes } from 'crypto'
import {
  type Transaction,
  type SignedTransaction,
  TransactionType,
  OnApplicationComplete,
  encodeTransaction,
  encodeSignedTransaction,
  getTransactionId,
  groupTransactions as groupTxns,
} from '@algorandfoundation/algokit-transact'
import { KmdClient } from '@algorandfoundation/kmd-client'
import * as ed from '@noble/ed25519'
import { AlgodClient } from '@algorandfoundation/algod-client'
import { IndexerClient } from '@algorandfoundation/indexer-client'
import { addressFromPublicKey, concatArrays, keyToMnemonic, mnemonicToKey, MnemonicError } from '@algorandfoundation/algokit-common'
import { AssetManager } from '../src/clients/asset-manager'
import { ClientManager } from '../src/clients/client-manager'
import { TransactionComposer } from '../src/transactions/composer'
import { waitForConfirmation, type SignerGetter, type TransactionSigner } from '../src/transactions/common'
import type { AssetCreateParams } from '../src/transactions/asset-config'
import type { AssetTransferParams } from '../src/transactions/asset-transfer'
import type { PaymentParams } from '../src/transactions/payment'

export interface AlgodTestConfig {
  algodBaseUrl: string
  algodApiToken?: string
  senderMnemonic?: string
}

export function getAlgodEnv(): AlgodTestConfig {
  return {
    algodBaseUrl: process.env.ALGOD_BASE_URL ?? 'http://localhost:4001',
    // Default token for localnet (Algorand sandbox / Algokit LocalNet)
    algodApiToken: process.env.ALGOD_API_TOKEN ?? 'a'.repeat(64),
    senderMnemonic: process.env.SENDER_MNEMONIC,
  }
}

// TODO: Revisit after account manager implementation
export async function getSenderMnemonic(): Promise<string> {
  if (process.env.SENDER_MNEMONIC) return process.env.SENDER_MNEMONIC
  const kmdBase = process.env.KMD_BASE_URL ?? 'http://localhost:4002'
  const kmdToken = process.env.KMD_API_TOKEN ?? 'a'.repeat(64)
  const walletPassword = process.env.KMD_WALLET_PASSWORD ?? ''
  const preferredWalletName = process.env.KMD_WALLET_NAME ?? 'unencrypted-default-wallet'

  const kmd = new KmdClient({
    baseUrl: kmdBase,
    apiToken: kmdToken,
  })

  const walletsResponse = await kmd.listWallets()
  const wallets = walletsResponse.wallets ?? []
  if (wallets.length === 0) {
    throw new Error('No KMD wallets available')
  }

  const wallet = wallets.find((w) => (w.name ?? '').toLowerCase() === preferredWalletName.toLowerCase()) ?? wallets[0]

  const walletId = wallet.id
  if (!walletId) {
    throw new Error('Wallet returned from KMD does not have an id')
  }

  const handleResponse = await kmd.initWalletHandleToken({
    body: {
      walletId,
      walletPassword,
    },
  })

  const walletHandleToken = handleResponse.walletHandleToken
  if (!walletHandleToken) {
    throw new Error('Failed to obtain wallet handle token from KMD')
  }

  try {
    const keysResponse = await kmd.listKeysInWallet({
      body: {
        walletHandleToken,
      },
    })
    let address = keysResponse.addresses?.[0]
    if (!address) {
      const generated = await kmd.generateKey({
        body: {
          walletHandleToken,
          displayMnemonic: false,
        },
      })
      address = generated.address ?? undefined
    }

    if (!address) {
      throw new Error('Unable to determine or generate a wallet key from KMD')
    }

    const exportResponse = await kmd.exportKey({
      body: {
        walletHandleToken,
        walletPassword,
        address,
      },
    })

    const exportedKey = exportResponse.privateKey
    if (!exportedKey) {
      throw new Error('KMD key export did not return a private key')
    }

    const secretKey = new Uint8Array(exportedKey)
    const mnemonic = keyToMnemonic(secretKey)
    if (!mnemonic) {
      throw new Error('Failed to convert secret key to mnemonic')
    }
    return mnemonic
  } finally {
    await kmd
      .releaseWalletHandleToken({
        body: {
          walletHandleToken,
        },
      })
      .catch(() => undefined)
  }
}

/**
 * Convenience helper: derive the sender account (address + keys) used for tests.
 * Returns:
 *  - address: Algorand address string
 *  - secretKey: 64-byte Ed25519 secret key (private + public)
 *  - mnemonic: the 25-word mnemonic
 */
export async function getSenderAccount(): Promise<{
  address: string
  secretKey: Uint8Array
  mnemonic: string
}> {
  const mnemonic = await getSenderMnemonic()
  let secretKey: Uint8Array
  try {
    secretKey = mnemonicToKey(mnemonic)
  } catch (error) {
    if (error instanceof MnemonicError) {
      throw new Error(`Failed to convert mnemonic to key: ${error.message}`)
    }
    throw error
  }
  const publicKey = secretKey.slice(32)
  const address = addressFromPublicKey(publicKey)
  return { address, secretKey, mnemonic }
}

export async function signTransaction(transaction: Transaction, secretKey: Uint8Array): Promise<SignedTransaction> {
  const encodedTxn = encodeTransaction(transaction)
  const signature = await ed.signAsync(encodedTxn, secretKey.slice(0, 32))

  return {
    transaction,
    signature,
  }
}

export function groupTransactions(transactions: Transaction[]): Transaction[] {
  return groupTxns(transactions)
}

export interface IndexerTestConfig {
  indexerBaseUrl: string
  indexerApiToken?: string
}

export interface CreatedAssetInfo {
  assetId: bigint
  txId: string
}

export interface CreatedAppInfo {
  appId: bigint
  txId: string
}

function decodeGenesisHash(genesisHash: string | Uint8Array): Uint8Array {
  if (genesisHash instanceof Uint8Array) {
    return new Uint8Array(genesisHash)
  }
  return new Uint8Array(Buffer.from(genesisHash, 'base64'))
}

async function submitTransaction(transaction: Transaction, algod: AlgodClient, secretKey: Uint8Array): Promise<{ txId: string }> {
  const signed = await signTransaction(transaction, secretKey)
  const raw = encodeSignedTransaction(signed)
  const txId = getTransactionId(transaction)
  await algod.rawTransaction({ body: raw })
  await waitForConfirmation(algod, txId, 10)
  return { txId }
}

export async function createTestApp(context: AlgorandFixtureContext): Promise<CreatedAppInfo> {
  const { address, secretKey } = context.creator
  const algod = context.algodClient
  const sp = await algod.transactionParams()

  const approvalProgramSource = '#pragma version 8\nint 1'
  const clearProgramSource = '#pragma version 8\nint 1'

  const compile = async (source: string) => {
    const result = await algod.tealCompile({ body: source })
    return new Uint8Array(Buffer.from(result.result, 'base64'))
  }

  const approvalProgram = await compile(approvalProgramSource)
  const clearProgram = await compile(clearProgramSource)

  const firstValid = sp.lastRound
  const lastValid = sp.lastRound + 1_000n

  const transaction: Transaction = {
    transactionType: TransactionType.AppCall,
    sender: address,
    firstValid,
    fee: sp.minFee,
    lastValid,
    genesisHash: decodeGenesisHash(sp.genesisHash),
    genesisId: sp.genesisId,
    appCall: {
      appId: 0n,
      onComplete: OnApplicationComplete.NoOp,
      approvalProgram,
      clearStateProgram: clearProgram,
      globalStateSchema: {
        numUints: 1,
        numByteSlices: 1,
      },
      localStateSchema: {
        numUints: 0,
        numByteSlices: 0,
      },
    },
  }

  const { txId } = await submitTransaction(transaction, algod, secretKey)

  const appId = (await algod.pendingTransactionInformation(txId)).appId
  if (!appId) {
    throw new Error('Application creation transaction confirmed without returning an app id')
  }

  return { appId, txId }
}

export function getIndexerEnv(): IndexerTestConfig {
  return {
    indexerBaseUrl: process.env.INDEXER_BASE_URL ?? 'http://localhost:8980',
    indexerApiToken: process.env.INDEXER_API_TOKEN ?? 'a'.repeat(64),
  }
}

export type TestAccount = {
  address: string
  secretKey: Uint8Array
}

type AlgorandFixtureContext = {
  algodClient: AlgodClient
  indexerClient: IndexerClient
  kmdClient: KmdClient
  assetManager: AssetManager
  creator: TestAccount
  signers: InMemorySignerRegistry
  newComposer: () => TransactionComposer
}

class InMemorySignerRegistry implements SignerGetter {
  private readonly signers = new Map<string, TransactionSigner>()
  private readonly secrets = new Map<string, Uint8Array>()
  private defaultSigner?: TransactionSigner
  private defaultSecret?: Uint8Array

  register(address: string, secretKey: Uint8Array): void {
    this.secrets.set(address, secretKey)
    this.signers.set(address, createTransactionSigner(secretKey))
  }

  setDefault(address: string, secretKey: Uint8Array): void {
    this.defaultSecret = secretKey
    this.defaultSigner = createTransactionSigner(secretKey)
    this.register(address, secretKey)
  }

  getSigner(address: string): TransactionSigner {
    const signer = this.signers.get(address) ?? this.defaultSigner
    if (!signer) {
      throw new Error(`No signer registered for address ${address}`)
    }
    return signer
  }

  getSecret(address: string): Uint8Array {
    const secret = this.secrets.get(address) ?? this.defaultSecret
    if (!secret) {
      throw new Error(`No secret key registered for address ${address}`)
    }
    return secret
  }
}

export function createClientManager(): ClientManager {
  const config = ClientManager.getConfigFromEnvironmentOrLocalNet()
  return new ClientManager(config)
}

export async function createAlgorandTestContext(): Promise<AlgorandFixtureContext> {
  const config = ClientManager.getConfigFromEnvironmentOrLocalNet()
  const algodClient = ClientManager.getAlgodClient(config.algodConfig)
  const indexerClient = ClientManager.getIndexerClient(config.indexerConfig)
  const kmdClient = ClientManager.getKmdClient(config.kmdConfig)
  const creatorAccount = await getSenderAccount()
  const creator: TestAccount = {
    address: creatorAccount.address,
    secretKey: creatorAccount.secretKey,
  }

  const signers = new InMemorySignerRegistry()
  signers.setDefault(creator.address, creator.secretKey)

  const newComposer = () =>
    new TransactionComposer({
      algodClient,
      signerGetter: signers,
    })

  const assetManager = new AssetManager(algodClient, newComposer)

  // TODO: Enhance and refine upon having all utils abstractions implemented (loosely based on rust algorand_fixture)
  return {
    algodClient,
    indexerClient,
    kmdClient,
    assetManager,
    creator,
    signers,
    newComposer,
  }
}

export async function createTestAsset(
  context: AlgorandFixtureContext,
  overrides: Partial<AssetCreateParams> = {},
): Promise<{ assetId: bigint; txnId: string }> {
  const composer = context.newComposer()
  const params: AssetCreateParams = {
    sender: context.creator.address,
    total: overrides.total ?? 1_000n,
    decimals: overrides.decimals ?? 0,
    unitName: overrides.unitName ?? 'TEST',
    assetName: overrides.assetName ?? 'Test Asset',
    defaultFrozen: overrides.defaultFrozen,
    manager: overrides.manager,
    reserve: overrides.reserve,
    freeze: overrides.freeze,
    clawback: overrides.clawback,
    note: new Uint8Array(randomBytes(8)),
  }

  composer.addAssetCreate({ ...params, ...overrides })
  const sendResult = await composer.send({ maxRoundsToWaitForConfirmation: 10 })

  const confirmation = sendResult.results.at(-1)?.confirmation
  if (confirmation?.assetId !== undefined && confirmation.assetId > 0) {
    const txnId = sendResult.results.at(-1)!.transactionId
    await waitForConfirmation(context.algodClient, txnId, 30)
    return { assetId: confirmation.assetId, txnId: txnId }
  }

  const txnId = sendResult.results.at(-1)?.transactionId
  if (!txnId) {
    throw new Error('Asset creation composer did not return a transaction id')
  }
  const pending = await waitForConfirmation(context.algodClient, txnId, 30)
  if (pending.assetId === undefined) {
    throw new Error('Pending transaction response missing assetId')
  }
  return { assetId: pending.assetId, txnId: txnId }
}

export async function createFundedAccount(context: AlgorandFixtureContext, initialFunding: bigint = 5_000_000n): Promise<TestAccount> {
  const privateKey = ed.utils.randomSecretKey()
  const publicKey = await ed.getPublicKeyAsync(privateKey)
  const secretKey = concatArrays(privateKey, publicKey)
  const address = addressFromPublicKey(publicKey)
  const account: TestAccount = {
    address,
    secretKey,
  }

  context.signers.register(account.address, account.secretKey)
  await sendPayment(context, {
    sender: context.creator.address,
    receiver: account.address,
    amount: initialFunding,
  })
  return account
}

export async function sendPayment(context: AlgorandFixtureContext, params: PaymentParams) {
  const composer = context.newComposer()
  composer.addPayment(params)
  await composer.send({ maxRoundsToWaitForConfirmation: 10 })
}

export async function transferAsset(context: AlgorandFixtureContext, params: AssetTransferParams) {
  const composer = context.newComposer()
  composer.addAssetTransfer(params)
  await composer.send({ maxRoundsToWaitForConfirmation: 10 })
}

function createTransactionSigner(secretKey: Uint8Array): TransactionSigner {
  const signSingle = async (transaction: Transaction): Promise<SignedTransaction> => {
    if (!transaction.sender) {
      throw new Error('Transaction missing sender')
    }
    return signTransaction(transaction, secretKey)
  }

  return {
    signTransaction: signSingle,
    signTransactions: async (transactions: Transaction[], indices: number[]) => {
      const signed: SignedTransaction[] = []
      for (const index of indices) {
        const transaction = transactions[index]
        if (!transaction) {
          throw new Error(`Missing transaction at index ${index}`)
        }
        signed.push(await signSingle(transaction))
      }
      return signed
    },
  }
}
