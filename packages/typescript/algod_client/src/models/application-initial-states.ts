import type { ModelMetadata } from '../core/model-runtime'
import type { ApplicationKvstorage } from './application-kvstorage'
import { ApplicationKvstorageMeta } from './application-kvstorage'

/**
 * An application's initial global/local/box states that were accessed during simulation.
 */
export type ApplicationInitialStates = {
  /**
   * Application index.
   */
  id: bigint

  /**
   * An application's initial local states tied to different accounts.
   */
  appLocals?: ApplicationKvstorage[]
  appGlobals?: ApplicationKvstorage
  appBoxes?: ApplicationKvstorage
}

export const ApplicationInitialStatesMeta: ModelMetadata = {
  name: 'ApplicationInitialStates',
  kind: 'object',
  fields: [
    {
      name: 'id',
      wireKey: 'id',
      optional: false,
      nullable: false,
      type: { kind: 'scalar', isBigint: true },
    },
    {
      name: 'appLocals',
      wireKey: 'app-locals',
      optional: true,
      nullable: false,
      type: { kind: 'array', item: { kind: 'model', meta: () => ApplicationKvstorageMeta } },
    },
    {
      name: 'appGlobals',
      wireKey: 'app-globals',
      optional: true,
      nullable: false,
      type: { kind: 'model', meta: () => ApplicationKvstorageMeta },
    },
    {
      name: 'appBoxes',
      wireKey: 'app-boxes',
      optional: true,
      nullable: false,
      type: { kind: 'model', meta: () => ApplicationKvstorageMeta },
    },
  ],
}
