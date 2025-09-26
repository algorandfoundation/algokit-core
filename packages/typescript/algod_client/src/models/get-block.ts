import type { ModelMetadata } from '../core/model-runtime'

export type GetBlock = {
  /**
   * Block header data.
   */
  block: Record<string, unknown>

  /**
   * Optional certificate object. This is only included when the format is set to message pack.
   */
  cert?: Record<string, unknown>
}

export const GetBlockMeta: ModelMetadata = {
  name: 'GetBlock',
  kind: 'object',
  fields: [
    {
      name: 'block',
      wireKey: 'block',
      optional: false,
      nullable: false,
      type: { kind: 'scalar' },
    },
    {
      name: 'cert',
      wireKey: 'cert',
      optional: true,
      nullable: false,
      type: { kind: 'scalar' },
    },
  ],
}
