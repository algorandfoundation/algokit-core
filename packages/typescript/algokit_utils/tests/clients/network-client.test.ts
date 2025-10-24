import { describe, expect, it } from 'vitest'
import { genesisIdIsLocalNet, genesisIdIsMainnet, genesisIdIsTestnet } from '../../src/clients/network-client'

describe('network helpers', () => {
  it('detects localnet genesis identifiers', () => {
    expect(genesisIdIsLocalNet('devnet-v1')).toBe(true)
    expect(genesisIdIsLocalNet('sandnet-v1')).toBe(true)
    expect(genesisIdIsLocalNet('mainnet-v1')).toBe(false)
  })

  it('detects testnet genesis identifiers', () => {
    expect(genesisIdIsTestnet('testnet-v1.0')).toBe(true)
    expect(genesisIdIsTestnet('testnet')).toBe(true)
    expect(genesisIdIsTestnet('dockernet-v1')).toBe(false)
  })

  it('detects mainnet genesis identifiers', () => {
    expect(genesisIdIsMainnet('mainnet-v1')).toBe(true)
    expect(genesisIdIsMainnet('mainnet')).toBe(true)
    expect(genesisIdIsMainnet('testnet-v1.0')).toBe(false)
  })
})
