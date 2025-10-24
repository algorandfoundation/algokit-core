import { describe, expect, it } from 'vitest'
import { createClientManager } from '../fixtures'

describe.sequential('ClientManager integration', () => {
  it('caches network details across sequential calls', async () => {
    const manager = createClientManager()

    const first = await manager.network()
    const second = await manager.network()
    const third = await manager.network()

    expect(second).toBe(first)
    expect(third).toBe(first)

    expect(first.genesisId.length).toBeGreaterThan(0)
    expect(first.genesisHash.length).toBeGreaterThan(0)
    const activeFlags = [first.isLocalnet, first.isTestnet, first.isMainnet].filter(Boolean)
    expect(activeFlags).toHaveLength(1)
  }, 30_000)

  it('deduplicates concurrent network lookups', async () => {
    const manager = createClientManager()

    const calls = await Promise.all(Array.from({ length: 6 }, () => manager.network()))

    calls.forEach((result) => {
      expect(result).toBe(calls[0])
    })
  }, 30_000)

  it('exposes convenience helpers resolved from the cached network details', async () => {
    const manager = createClientManager()
    const network = await manager.network()

    const [isLocal, isTest, isMain] = await Promise.all([manager.isLocalNet(), manager.isTestNet(), manager.isMainNet()])

    expect(isLocal).toBe(network.isLocalnet)
    expect(isTest).toBe(network.isTestnet)
    expect(isMain).toBe(network.isMainnet)
  }, 30_000)

  it('validates network details structure for localnet', async () => {
    const manager = createClientManager()
    const details = await manager.network()

    // Verify structure
    expect(details.genesisId.length).toBeGreaterThan(0)
    expect(details.genesisHash.length).toBeGreaterThan(0)

    // Verify exactly one network type is detected
    const networkFlags = [details.isLocalnet, details.isTestnet, details.isMainnet]
    const activeNetworks = networkFlags.filter(Boolean)
    expect(activeNetworks).toHaveLength(1)

    // Should detect as localnet for local config
    expect(details.isLocalnet).toBe(true)
  }, 30_000)
})
