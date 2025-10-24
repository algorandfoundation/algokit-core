import { expect, it, describe } from 'vitest'
import { IndexerClient } from '@algorandfoundation/indexer-client'
import { getIndexerEnv, createTestAsset, createAlgorandTestContext } from '../fixtures'
import { waitForIndexerTransaction } from '../../src'

describe('Indexer search transactions', () => {
  it('should search for transactions', async () => {
    const context = await createAlgorandTestContext()
    const { assetId, txnId } = await createTestAsset(context)
    const env = getIndexerEnv()
    const client = new IndexerClient({ baseUrl: env.indexerBaseUrl, apiToken: env.indexerApiToken ?? undefined })

    await waitForIndexerTransaction(client, txnId)

    const res = await client.searchForTransactions()
    expect(res).toHaveProperty('transactions')
    expect(res.transactions && res.transactions.length).toBeGreaterThan(0)

    const assetTxns = await client.searchForTransactions({ txType: 'acfg', assetId: assetId })
    expect(assetTxns).toHaveProperty('transactions')
    expect(assetTxns.transactions[0].createdAssetIndex).toBe(assetId)
  })
})
