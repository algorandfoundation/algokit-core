import {
  AlgodClient,
  ApiError,
  FetchHttpRequest,
  type BaseHttpRequest,
  type ClientConfig as HttpClientConfig,
} from '@algorandfoundation/algod-client'
import { IndexerClient } from '@algorandfoundation/indexer-client'
import { KmdClient } from '@algorandfoundation/kmd-client'
import { Buffer } from 'buffer'

import {
  AlgoClientConfig,
  AlgoConfig,
  AlgorandService,
  NetworkDetails,
  TokenHeader,
  genesisIdIsLocalNet,
  genesisIdIsMainnet,
  genesisIdIsTestnet,
} from './network-client'

export interface ClientManagerClients {
  algod: AlgodClient
  indexer?: IndexerClient
  kmd?: KmdClient
}

interface NetworkCache {
  value?: NetworkDetails
  promise?: Promise<NetworkDetails>
}

type HttpClientFactoryResult = {
  clientConfig: HttpClientConfig
  request?: BaseHttpRequest
}

type HttpClientFactory = (config: AlgoClientConfig, defaultHeaderName: string) => HttpClientFactoryResult

export class ClientManager {
  private static httpClientFactory?: HttpClientFactory
  private readonly algodClient: AlgodClient
  private readonly indexerClient?: IndexerClient
  private readonly kmdClient?: KmdClient
  private readonly networkCache: NetworkCache = {}

  constructor(clientsOrConfig: ClientManagerClients | AlgoConfig) {
    const clients =
      'algod' in clientsOrConfig
        ? clientsOrConfig
        : {
            algod: ClientManager.getAlgodClient(clientsOrConfig.algodConfig),
            indexer: clientsOrConfig.indexerConfig ? ClientManager.getIndexerClient(clientsOrConfig.indexerConfig) : undefined,
            kmd: clientsOrConfig.kmdConfig ? ClientManager.getKmdClient(clientsOrConfig.kmdConfig) : undefined,
          }

    this.algodClient = clients.algod
    this.indexerClient = clients.indexer
    this.kmdClient = clients.kmd
  }

  /** Returns an Algod API client. */
  get algod(): AlgodClient {
    return this.algodClient
  }

  /** Returns an Indexer API client or throws if not configured. */
  get indexer(): IndexerClient {
    if (!this.indexerClient) {
      throw new Error('Attempt to use Indexer client without configuring one')
    }
    return this.indexerClient
  }

  /** Returns an Indexer API client if present. */
  get indexerIfPresent(): IndexerClient | undefined {
    return this.indexerClient
  }

  /** Returns a KMD API client or throws if not configured. */
  get kmd(): KmdClient {
    if (!this.kmdClient) {
      throw new Error('Attempt to use KMD client without configuring one')
    }
    return this.kmdClient
  }

  /** Returns a KMD API client if present. */
  get kmdIfPresent(): KmdClient | undefined {
    return this.kmdClient
  }

  /** Get details about the current network. */
  async network(): Promise<NetworkDetails> {
    if (this.networkCache.value) {
      return this.networkCache.value
    }

    if (!this.networkCache.promise) {
      this.networkCache.promise = this.fetchNetworkDetails()
        .then((details) => {
          this.networkCache.value = details
          this.networkCache.promise = undefined
          return details
        })
        .catch((error) => {
          this.networkCache.promise = undefined
          throw error
        })
    }

    return this.networkCache.promise
  }

  /** Returns true if the given genesis ID is associated with a LocalNet network. */
  static genesisIdIsLocalNet(genesisId: string): boolean {
    return genesisIdIsLocalNet(genesisId)
  }

  /** Returns true if the current network is LocalNet. */
  async isLocalNet(): Promise<boolean> {
    const network = await this.network()
    return network.isLocalnet
  }

  /** Returns true if the current network is TestNet. */
  async isTestNet(): Promise<boolean> {
    const network = await this.network()
    return network.isTestnet
  }

  /** Returns true if the current network is MainNet. */
  async isMainNet(): Promise<boolean> {
    const network = await this.network()
    return network.isMainnet
  }

  /**
   * TODO: Provide TestNet dispenser helper once dependencies are ported from legacy algokit-utils-ts.
   */
  getTestNetDispenser(_params: unknown): never {
    throw new Error('TODO: getTestNetDispenser is not yet implemented in the TypeScript ClientManager')
  }

  /**
   * TODO: Provide environment-based TestNet dispenser helper once dependencies are ported from legacy algokit-utils-ts.
   */
  getTestNetDispenserFromEnvironment(_params?: unknown): never {
    throw new Error('TODO: getTestNetDispenserFromEnvironment is not yet implemented in the TypeScript ClientManager')
  }

  /**
   * TODO: Provide app factory support once app client abstractions are ported from legacy algokit-utils-ts.
   */
  getAppFactory(_params: unknown): never {
    throw new Error('TODO: getAppFactory is not yet implemented in the TypeScript ClientManager')
  }

  /**
   * TODO: Provide app client lookup by creator and name after porting legacy algokit-utils-ts.
   */
  async getAppClientByCreatorAndName(_params: unknown): Promise<never> {
    throw new Error('TODO: getAppClientByCreatorAndName is not yet implemented in the TypeScript ClientManager')
  }

  /**
   * TODO: Provide app client lookup by ID after porting legacy algokit-utils-ts.
   */
  getAppClientById(_params: unknown): never {
    throw new Error('TODO: getAppClientById is not yet implemented in the TypeScript ClientManager')
  }

  /**
   * TODO: Provide app client lookup by network after porting legacy algokit-utils-ts.
   */
  async getAppClientByNetwork(_params: unknown): Promise<never> {
    throw new Error('TODO: getAppClientByNetwork is not yet implemented in the TypeScript ClientManager')
  }

  /**
   * TODO: Provide typed app client lookup after porting legacy algokit-utils-ts.
   */
  async getTypedAppClientByCreatorAndName(_typedClient: unknown, _params: unknown): Promise<never> {
    throw new Error('TODO: getTypedAppClientByCreatorAndName is not yet implemented in the TypeScript ClientManager')
  }

  /**
   * TODO: Provide typed app client lookup by ID after porting legacy algokit-utils-ts.
   */
  getTypedAppClientById(_typedClient: unknown, _params: unknown): never {
    throw new Error('TODO: getTypedAppClientById is not yet implemented in the TypeScript ClientManager')
  }

  /**
   * TODO: Provide typed app client lookup by network after porting legacy algokit-utils-ts.
   */
  async getTypedAppClientByNetwork(_typedClient: unknown, _params?: unknown): Promise<never> {
    throw new Error('TODO: getTypedAppClientByNetwork is not yet implemented in the TypeScript ClientManager')
  }

  /**
   * TODO: Provide typed app factory construction after porting legacy algokit-utils-ts.
   */
  getTypedAppFactory(_typedFactory: unknown, _params?: unknown): never {
    throw new Error('TODO: getTypedAppFactory is not yet implemented in the TypeScript ClientManager')
  }

  /**
   * Derive configuration from the environment if possible, otherwise default to a localnet configuration.
   */
  static getConfigFromEnvironmentOrLocalNet(): AlgoConfig {
    if (!process || !process.env) {
      throw new Error('Attempt to get default client configuration from a non Node.js context; supply the config instead')
    }
    const [algodConfig, indexerConfig, kmdConfig] = process.env.ALGOD_SERVER
      ? [
          ClientManager.getAlgodConfigFromEnvironment(),
          process.env.INDEXER_SERVER ? ClientManager.getIndexerConfigFromEnvironment() : undefined,
          !process.env.ALGOD_SERVER.includes('mainnet') && !process.env.ALGOD_SERVER.includes('testnet')
            ? { ...ClientManager.getAlgodConfigFromEnvironment(), port: process?.env?.KMD_PORT ?? '4002' }
            : undefined,
        ]
      : [
          ClientManager.getDefaultLocalNetConfig(AlgorandService.Algod),
          ClientManager.getDefaultLocalNetConfig(AlgorandService.Indexer),
          ClientManager.getDefaultLocalNetConfig(AlgorandService.Kmd),
        ]

    return {
      algodConfig,
      indexerConfig,
      kmdConfig,
    }
  }

  /**
   * Returns Indexer configuration derived from environment variables.
   * @throws Error if required environment variables are missing.
   */
  static getIndexerConfigFromEnvironment(): AlgoClientConfig {
    const server = process.env.INDEXER_SERVER
    if (!server) {
      throw new Error('INDEXER_SERVER environment variable not found')
    }

    const port = this.parsePort(process.env.INDEXER_PORT)
    const token = process.env.INDEXER_TOKEN

    return {
      server,
      port,
      token: token ?? undefined,
    }
  }

  /**
   * Returns Algod configuration derived from environment variables.
   * @throws Error if required environment variables are missing.
   */
  static getAlgodConfigFromEnvironment(): AlgoClientConfig {
    const server = process.env.ALGOD_SERVER
    if (!server) {
      throw new Error('ALGOD_SERVER environment variable not found')
    }

    const port = this.parsePort(process.env.ALGOD_PORT)
    const token = process.env.ALGOD_TOKEN

    return {
      server,
      port,
      token: token ?? undefined,
    }
  }

  /**
   * Returns KMD configuration derived from environment variables.
   * Falls back to ALGOD_* variables if KMD_* are not provided.
   * @throws Error if no server can be determined.
   */
  static getKmdConfigFromEnvironment(fallbackAlgodConfig?: AlgoClientConfig): AlgoClientConfig {
    const server = process.env.KMD_SERVER ?? fallbackAlgodConfig?.server ?? process.env.ALGOD_SERVER
    if (!server) {
      throw new Error('KMD_SERVER environment variable not found')
    }

    const port = this.parsePort(process.env.KMD_PORT) ?? fallbackAlgodConfig?.port ?? this.parsePort(process.env.ALGOD_PORT) ?? 4002

    const token = process.env.KMD_TOKEN ?? process.env.ALGOD_TOKEN

    return {
      server,
      port,
      token: token ?? undefined,
    }
  }

  /** Returns the Algorand configuration to point to the free tier of the AlgoNode service.
   *
   * @param network Which network to connect to - TestNet or MainNet
   * @param config Which algod config to return - Algod or Indexer
   * @returns The AlgoNode client configuration
   * @example
   * ```typescript
   * const config = ClientManager.getAlgoNodeConfig('testnet', 'algod')
   * ```
   */
  static getAlgoNodeConfig(network: string, service: AlgorandService): AlgoClientConfig {
    if (service === AlgorandService.Kmd) {
      throw new Error('KMD is not available on algonode')
    }

    const subdomain = service === AlgorandService.Algod ? 'api' : 'idx'

    return {
      server: `https://${network}-${subdomain}.4160.nodely.dev`,
      port: 443,
    }
  }

  /** Returns the Algorand configuration to point to the default LocalNet.
   *
   * @param configOrPort Which algod config to return - algod, kmd, or indexer OR a port number
   * @returns The LocalNet client configuration
   * @example
   * ```typescript
   * const config = ClientManager.getDefaultLocalNetConfig('algod')
   * ```
   */
  public static getDefaultLocalNetConfig(configOrPort: AlgorandService | number): AlgoClientConfig {
    return {
      server: `http://localhost`,
      port:
        configOrPort === AlgorandService.Algod
          ? 4001
          : configOrPort === AlgorandService.Indexer
            ? 8980
            : configOrPort === AlgorandService.Kmd
              ? 4002
              : configOrPort,
      token: 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa',
    }
  }

  /**
   * Creates an Algod client for the given configuration.
   */
  static getAlgodClient(config: AlgoClientConfig): AlgodClient {
    const { clientConfig, request } = this.createHttpClientComponents(config, 'X-Algo-API-Token')
    return new AlgodClient(clientConfig, request)
  }

  /**
   * Creates an Indexer client for the given configuration.
   */
  static getIndexerClient(config: AlgoClientConfig): IndexerClient {
    const { clientConfig, request } = this.createHttpClientComponents(config, 'X-Indexer-API-Token')
    return new IndexerClient(clientConfig, request)
  }

  /**
   * Creates a KMD client for the given configuration.
   */
  static getKmdClient(config: AlgoClientConfig): KmdClient {
    const { clientConfig, request } = this.createHttpClientComponents(config, 'X-KMD-API-Token')
    return new KmdClient(clientConfig, request)
  }

  /**
   * Creates an Algod client from environment variables.
   */
  static getAlgodClientFromEnvironment(): AlgodClient {
    return this.getAlgodClient(this.getAlgodConfigFromEnvironment())
  }

  /**
   * Creates an Indexer client from environment variables.
   */
  static getIndexerClientFromEnvironment(): IndexerClient {
    return this.getIndexerClient(this.getIndexerConfigFromEnvironment())
  }

  /**
   * Creates a KMD client from environment variables.
   */
  static getKmdClientFromEnvironment(): KmdClient {
    const algodConfig = this.safeGetConfig(this.getAlgodConfigFromEnvironment.bind(this))
    return this.getKmdClient(this.getKmdConfigFromEnvironment(algodConfig))
  }

  private async fetchNetworkDetails(): Promise<NetworkDetails> {
    try {
      const params = await this.algodClient.transactionParams()
      const genesisId = params.genesisId ?? 'unknown'
      const genesisHash = Buffer.from(params.genesisHash ?? new Uint8Array()).toString('base64')

      return {
        isTestnet: genesisIdIsTestnet(genesisId),
        isMainnet: genesisIdIsMainnet(genesisId),
        isLocalnet: genesisIdIsLocalNet(genesisId),
        genesisId,
        genesisHash,
      }
    } catch (error) {
      if (error instanceof ApiError) {
        throw new Error(`Failed to fetch network details: ${error.message}`)
      }
      throw error
    }
  }

  private static createHttpClientComponents(config: AlgoClientConfig, defaultHeaderName: string): HttpClientFactoryResult {
    if (this.httpClientFactory) {
      return this.httpClientFactory(config, defaultHeaderName)
    }
    const clientConfig = this.buildHttpClientConfig(config, defaultHeaderName)
    return {
      clientConfig,
      request: new FetchHttpRequest(clientConfig),
    }
  }

  private static buildHttpClientConfig(config: AlgoClientConfig, defaultHeaderName: string): HttpClientConfig {
    const baseUrl = this.buildBaseUrl(config)
    const headers = this.buildHeaders(config.token, defaultHeaderName)
    return headers ? { baseUrl, headers } : { baseUrl }
  }

  /**
   * Configure a custom HTTP client factory, e.g. to integrate the retry-enabled HTTP layer.
   */
  static configureHttpClientFactory(factory: HttpClientFactory | undefined): void {
    this.httpClientFactory = factory
  }

  private static buildHeaders(token: TokenHeader | undefined, defaultHeaderName: string): Record<string, string> | undefined {
    if (!token) {
      return undefined
    }

    if (typeof token === 'string') {
      return { [defaultHeaderName]: token }
    }

    return { ...token }
  }

  private static buildBaseUrl(config: AlgoClientConfig): string {
    const { server, port } = config
    if (port === undefined || port === null || port === '') {
      return server
    }

    const portString = typeof port === 'string' ? port : port.toString()

    try {
      const url = new URL(server)
      url.port = portString
      const normalized = url.toString()
      return normalized.endsWith('/') ? normalized.slice(0, -1) : normalized
    } catch {
      if (/:[0-9]+$/.test(server)) {
        return server
      }
      return `${server}:${portString}`
    }
  }

  private static parsePort(value: string | number | undefined | null): number | undefined {
    if (value === undefined || value === null || value === '') {
      return undefined
    }
    if (typeof value === 'number') {
      return value
    }
    const parsed = Number(value)
    return Number.isNaN(parsed) ? undefined : parsed
  }

  private static safeGetConfig(getter: () => AlgoClientConfig): AlgoClientConfig | undefined {
    try {
      return getter()
    } catch {
      return undefined
    }
  }
}
