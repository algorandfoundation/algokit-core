import { MAX_TX_GROUP_SIZE } from '@algorandfoundation/algokit-common'
import { describe, expect, it } from 'vitest'
import { createAlgorandTestContext, createFundedAccount, createTestAsset, transferAsset } from '../fixtures'

const TEST_TIMEOUT = 120_000

describe.sequential('AssetManager integration', () => {
  it(
    'retrieves asset information by id',
    async () => {
      const context = await createAlgorandTestContext()
      const { assetId } = await createTestAsset(context, { assetName: 'AssetManager E2E', unitName: 'AME2E' })

      const info = await context.assetManager.getById(assetId)

      expect(info.assetId).toBe(assetId)
      expect(info.creator).toBe(context.creator.address)
      expect(info.total).toBe(1_000n)
      expect(info.decimals).toBe(0)
      expect(info.assetName).toBe('AssetManager E2E')
      expect(info.unitName).toBe('AME2E')
    },
    TEST_TIMEOUT,
  )

  it(
    'maps missing assets to ASSET_NOT_FOUND errors',
    async () => {
      const context = await createAlgorandTestContext()

      await expect(context.assetManager.getById(9_999_999_999n)).rejects.toMatchObject({
        message: 'Asset not found: 9999999999',
      })
    },
    TEST_TIMEOUT,
  )

  it(
    'retrieves account holdings for opted-in creator',
    async () => {
      const context = await createAlgorandTestContext()
      const { assetId } = await createTestAsset(context)

      const accountInfo = await context.assetManager.getAccountInformation(context.creator.address, assetId)

      expect(accountInfo.assetHolding?.assetId).toBe(assetId)
      expect(accountInfo.assetHolding?.amount).toBe(1_000n)
    },
    TEST_TIMEOUT,
  )

  it(
    'raises NOT_OPTED_IN when account has not opted in',
    async () => {
      const context = await createAlgorandTestContext()
      const { assetId } = await createTestAsset(context)
      const account = await createFundedAccount(context)

      await expect(context.assetManager.getAccountInformation(account.address, assetId)).rejects.toMatchObject({
        message: expect.stringContaining('is not opted into asset'),
      })
    },
    TEST_TIMEOUT,
  )

  it(
    'bulk opt in opts into each requested asset',
    async () => {
      const context = await createAlgorandTestContext()
      const assets = (await Promise.all([createTestAsset(context), createTestAsset(context)])).map((a) => a.assetId)
      const account = await createFundedAccount(context)

      const results = await context.assetManager.bulkOptIn(account.address, assets)

      expect(results).toHaveLength(assets.length)
      for (const [index, result] of results.entries()) {
        expect(result.assetId).toBe(assets[index])
        const info = await context.assetManager.getAccountInformation(account.address, assets[index])
        expect(info.assetHolding?.assetId).toBe(assets[index])
        expect(info.assetHolding?.amount).toBe(0n)
      }
    },
    TEST_TIMEOUT,
  )

  it(
    'bulk opt in splits batches above the max group size',
    async () => {
      const context = await createAlgorandTestContext()
      const assetCount = MAX_TX_GROUP_SIZE + 3
      const assetIds: bigint[] = []
      for (let i = 0; i < assetCount; i++) {
        assetIds.push((await createTestAsset(context)).assetId)
      }
      const account = await createFundedAccount(context)

      const results = await context.assetManager.bulkOptIn(account.address, assetIds)

      expect(results).toHaveLength(assetCount)
      expect(results.map((r) => r.assetId)).toEqual(assetIds)
    },
    TEST_TIMEOUT,
  )

  it(
    'bulk opt in returns an empty collection when no assets provided',
    async () => {
      const context = await createAlgorandTestContext()
      const account = await createFundedAccount(context)

      const results = await context.assetManager.bulkOptIn(account.address, [])

      expect(results).toEqual([])
    },
    TEST_TIMEOUT,
  )

  it(
    'bulk opt out removes holdings and closes to the creator',
    async () => {
      const context = await createAlgorandTestContext()
      const assetIds = (await Promise.all([createTestAsset(context), createTestAsset(context)])).map((a) => a.assetId)
      const account = await createFundedAccount(context)

      await context.assetManager.bulkOptIn(account.address, assetIds)

      const results = await context.assetManager.bulkOptOut(account.address, assetIds, true)

      expect(results).toHaveLength(assetIds.length)
      for (const assetId of assetIds) {
        await expect(context.assetManager.getAccountInformation(account.address, assetId)).rejects.toMatchObject({
          message: expect.stringContaining('is not opted into asset'),
        })
      }
    },
    TEST_TIMEOUT,
  )

  it(
    'bulk opt out splits batches appropriately',
    async () => {
      const context = await createAlgorandTestContext()
      const assetCount = MAX_TX_GROUP_SIZE + 2
      const assetIds: bigint[] = []
      for (let i = 0; i < assetCount; i++) {
        assetIds.push((await createTestAsset(context)).assetId)
      }
      const account = await createFundedAccount(context)

      await context.assetManager.bulkOptIn(account.address, assetIds)

      const results = await context.assetManager.bulkOptOut(account.address, assetIds, true)

      expect(results).toHaveLength(assetCount)
      expect(results.map((r) => r.assetId)).toEqual(assetIds)
    },
    TEST_TIMEOUT,
  )

  it(
    'bulk opt out returns an empty collection for empty requests',
    async () => {
      const context = await createAlgorandTestContext()
      const account = await createFundedAccount(context)

      const results = await context.assetManager.bulkOptOut(account.address, [], true)

      expect(results).toEqual([])
    },
    TEST_TIMEOUT,
  )

  it(
    'bulk opt out rejects when balance check detects non-zero balance',
    async () => {
      const context = await createAlgorandTestContext()
      const { assetId } = await createTestAsset(context)
      const account = await createFundedAccount(context)

      await context.assetManager.bulkOptIn(account.address, [assetId])
      await transferAsset(context, {
        sender: context.creator.address,
        receiver: account.address,
        amount: 10n,
        assetId,
      })

      await expect(context.assetManager.bulkOptOut(account.address, [assetId], true)).rejects.toMatchObject({
        message: expect.stringContaining('has non-zero balance'),
      })
    },
    TEST_TIMEOUT,
  )

  it(
    'bulk opt out can override the balance check and close out remaining balance',
    async () => {
      const context = await createAlgorandTestContext()
      const { assetId } = await createTestAsset(context)
      const account = await createFundedAccount(context)

      await context.assetManager.bulkOptIn(account.address, [assetId])
      await transferAsset(context, {
        sender: context.creator.address,
        receiver: account.address,
        amount: 5n,
        assetId,
      })

      const results = await context.assetManager.bulkOptOut(account.address, [assetId], false)

      expect(results).toHaveLength(1)
      await expect(context.assetManager.getAccountInformation(account.address, assetId)).rejects.toMatchObject({
        message: expect.stringContaining('is not opted into asset'),
      })
    },
    TEST_TIMEOUT,
  )
})
