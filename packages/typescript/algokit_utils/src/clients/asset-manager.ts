import { AccountAssetInformation, AlgodClient, ApiError } from '@algorandfoundation/algod-client'
import { AssetOptInParams, AssetOptOutParams } from '../transactions/asset-transfer'
import { TransactionComposer } from '../transactions/composer'
import { MAX_TX_GROUP_SIZE } from '@algorandfoundation/algokit-common'
import { chunkArray, createError } from '../util'

/** Individual result from performing a bulk opt-in or bulk opt-out for an account against a series of assets. */
export interface BulkAssetOptInOutResult {
  /** The ID of the asset opted into / out of */
  assetId: bigint
  /** The transaction ID of the resulting opt in / out */
  transactionId: string
}

/** Information about an Algorand Standard Asset (ASA).
 *
 * This type provides a flattened, developer-friendly interface to asset information
 * that aligns with TypeScript and Python implementations.
 */
export interface AssetInformation {
  /** The ID of the asset. */
  assetId: bigint

  /** The address of the account that created the asset.
   *
   * This is the address where the parameters for this asset can be found,
   * and also the address where unwanted asset units can be sent when
   * closing out an asset position and opting-out of the asset.
   */
  creator: string

  /** The total amount of the smallest divisible (decimal) units that were created of the asset.
   *
   * For example, if `decimals` is, say, 2, then for every 100 `total` there is 1 whole unit.
   */
  total: bigint

  /** The amount of decimal places the asset was created with.
   *
   * * If 0, the asset is not divisible;
   * * If 1, the base unit of the asset is in tenths;
   * * If 2, the base unit of the asset is in hundredths;
   * * If 3, the base unit of the asset is in thousandths;
   * * and so on up to 19 decimal places.
   */
  decimals: number

  /** Whether the asset was frozen by default for all accounts.
   *
   * If `true` then for anyone apart from the creator to hold the
   * asset it needs to be unfrozen per account using an asset freeze
   * transaction from the `freeze` account.
   */
  defaultFrozen?: boolean

  /** The address of the optional account that can manage the configuration of the asset and destroy it.
   *
   * If not set the asset is permanently immutable.
   */
  manager?: string

  /** The address of the optional account that holds the reserve (uncirculated supply) units of the asset.
   *
   * This address has no specific authority in the protocol itself and is informational only.
   *
   * Some standards like [ARC-19](https://github.com/algorandfoundation/ARCs/blob/main/ARCs/arc-0019.md)
   * rely on this field to hold meaningful data.
   *
   * It can be used in the case where you want to signal to holders of your asset that the uncirculated units
   * of the asset reside in an account that is different from the default creator account.
   *
   * If not set the field is permanently empty.
   */
  reserve?: string

  /** The address of the optional account that can be used to freeze or unfreeze holdings of this asset for any account.
   *
   * If empty, freezing is not permitted.
   *
   * If not set the field is permanently empty.
   */
  freeze?: string

  /** The address of the optional account that can clawback holdings of this asset from any account.
   *
   * The clawback account has the ability to **unconditionally take assets from any account**.
   *
   * If empty, clawback is not permitted.
   *
   * If not set the field is permanently empty.
   */
  clawback?: string

  /** The optional name of the unit of this asset (e.g. ticker name).
   *
   * Max size is 8 bytes.
   */
  unitName?: string

  /** The optional name of the unit of this asset as bytes.
   *
   * Max size is 8 bytes.
   */
  unitNameAsBytes?: Uint8Array

  /** The optional name of the asset.
   *
   * Max size is 32 bytes.
   */
  assetName?: string

  /** The optional name of the asset as bytes.
   *
   * Max size is 32 bytes.
   */
  assetNameAsBytes?: Uint8Array

  /** Optional URL where more information about the asset can be retrieved (e.g. metadata).
   *
   * Max size is 96 bytes.
   */
  url?: string

  /** Optional URL where more information about the asset can be retrieved as bytes.
   *
   * Max size is 96 bytes.
   */
  urlAsBytes?: Uint8Array

  /** 32-byte hash of some metadata that is relevant to the asset and/or asset holders.
   *
   * The format of this metadata is up to the application.
   */
  metadataHash?: Uint8Array
}

/** Manages Algorand Standard Assets. */
export class AssetManager {
  private readonly algodClient: AlgodClient
  private readonly newComposer: () => TransactionComposer

  constructor(algodClient: AlgodClient, newComposer: () => TransactionComposer) {
    this.algodClient = algodClient
    this.newComposer = newComposer
  }

  /** Get asset information by asset ID
   * Returns a convenient, flattened view of the asset information.
   */
  async getById(assetId: bigint): Promise<AssetInformation> {
    try {
      const asset = await this.algodClient.getAssetById(assetId)

      return {
        assetId: asset.index,
        creator: asset.params.creator,
        total: asset.params.total,
        decimals: Number(asset.params.decimals),
        defaultFrozen: asset.params.defaultFrozen,
        manager: asset.params.manager,
        reserve: asset.params.reserve,
        freeze: asset.params.freeze,
        clawback: asset.params.clawback,
        unitName: asset.params.unitName,
        unitNameAsBytes: asset.params.unitNameB64,
        assetName: asset.params.name,
        assetNameAsBytes: asset.params.nameB64,
        url: asset.params.url,
        urlAsBytes: asset.params.urlB64,
        metadataHash: asset.params.metadataHash,
      }
    } catch (error) {
      if (error instanceof ApiError && error.status === 404) {
        throw createError(`Asset not found: ${assetId}`, error)
      }
      throw createError(`Failed to fetch asset information for asset ${assetId}`, error)
    }
  }

  /** Get account's asset information.
   * Returns the raw algod AccountAssetInformation type.
   * Access asset holding via `account_info.asset_holding` and asset params via `account_info.asset_params`.
   */
  async getAccountInformation(sender: string, assetId: bigint): Promise<AccountAssetInformation> {
    try {
      return await this.algodClient.accountAssetInformation(sender, assetId, { format: 'json' })
    } catch (error) {
      if (error instanceof ApiError) {
        if (error.status === 404) {
          throw createError(`Account ${sender} is not opted into asset ${assetId}`, error)
        }
        if (error.status === 400) {
          throw createError(`Account not found: ${sender}`, error)
        }
      }
      throw createError(`Failed to fetch account asset information for account ${sender} and asset ${assetId}`, error)
    }
  }

  async bulkOptIn(account: string, assetIds: bigint[]): Promise<BulkAssetOptInOutResult[]> {
    if (assetIds.length === 0) {
      return []
    }

    // Ignore duplicate asset IDs while preserving input order
    const uniqueIds = [...new Set(assetIds)]

    const results: BulkAssetOptInOutResult[] = []

    for (const batch of chunkArray(uniqueIds, MAX_TX_GROUP_SIZE)) {
      const composer = this.newComposer()

      for (const assetId of batch) {
        const params: AssetOptInParams = {
          sender: account,
          assetId,
        }

        try {
          composer.addAssetOptIn(params)
        } catch (error) {
          throw createError(`Failed to add opt-in for asset ${assetId}`, error)
        }
      }

      try {
        const result = await composer.send()

        if (result.results.length !== batch.length) {
          throw new Error(`Composer returned an unexpected number of results (expected ${batch.length}, actual ${result.results.length})`)
        }

        batch.forEach((assetId, index) => {
          results.push({
            assetId,
            transactionId: result.results[index].transactionId,
          })
        })
      } catch (error) {
        throw createError('Failed to submit opt-in transactions', error)
      }
    }

    return results
  }

  async bulkOptOut(account: string, assetIds: bigint[], ensureZeroBalance?: boolean): Promise<BulkAssetOptInOutResult[]> {
    if (assetIds.length === 0) {
      return []
    }

    // Ignore duplicate asset IDs while preserving input order
    const uniqueIds = [...new Set(assetIds)]

    const shouldCheckBalance = ensureZeroBalance ?? false
    const results: BulkAssetOptInOutResult[] = []

    if (shouldCheckBalance) {
      for (const assetId of uniqueIds) {
        const accountInfo = await this.getAccountInformation(account, assetId)

        const balance = accountInfo.assetHolding?.amount ?? 0n
        if (balance > 0n) {
          throw new Error(`Account ${account} has non-zero balance (${balance}) for asset ${assetId}`)
        }
      }
    }

    // Precompute creator cache for all assetIds before batching
    const creatorCache = new Map<bigint, string>()
    for (const assetId of uniqueIds) {
      const assetInfo = await this.getById(assetId)
      creatorCache.set(assetId, assetInfo.creator)
    }

    // Prepare stable pairs to preserve input order
    const assetCreatorPairs = uniqueIds.map((assetId) => [assetId, creatorCache.get(assetId)!] as const)

    for (const batch of chunkArray(assetCreatorPairs, MAX_TX_GROUP_SIZE)) {
      const composer = this.newComposer()

      for (const [assetId, creator] of batch) {
        const params: AssetOptOutParams = {
          sender: account,
          assetId,
          closeRemainderTo: creator,
        }

        try {
          composer.addAssetOptOut(params)
        } catch (error) {
          throw createError(`Failed to add opt-out for asset ${assetId}`, error)
        }
      }

      try {
        const result = await composer.send()

        if (result.results.length !== batch.length) {
          throw new Error(`Composer returned an unexpected number of results (expected ${batch.length}, actual ${result.results.length})`)
        }

        batch.forEach(([assetId], index) => {
          results.push({
            assetId,
            transactionId: result.results[index].transactionId,
          })
        })
      } catch (error) {
        throw createError('Failed to submit opt-out transactions', error)
      }
    }

    return results
  }
}
