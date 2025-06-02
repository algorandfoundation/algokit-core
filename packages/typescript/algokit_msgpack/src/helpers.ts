import * as algokit_msgpack from '../pkg';

/**
 * Exception thrown when FFI functionality is not implemented
 */
export class FFINotImplementedError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'FFINotImplementedError';
  }
}

/**
 * Convert CamelCase or PascalCase to snake_case.
 * 
 * Examples:
 *   TransactionParams200Response -> transaction_params_200_response
 *   Account -> account
 *   HTTPResponseCode -> http_response_code
 */
function camelToSnakeCase(name: string): string {
  name = name.replace(/(.)([A-Z][a-z]+)/g, '$1_$2');
  name = name.replace(/([a-z0-9])([A-Z])/g, '$1_$2');
  return name.toLowerCase();
}

/**
 * Get the appropriate FFI function for a given model and operation
 */
function getFfiFunc(
  pkgModule: any,
  baseName: string,
  modelNameLower: string,
  direction: string,
  ffiType: string
): Function | null {
  let funcNameSpecific: string;
  
  if (ffiType === 'json') {
    if (direction === 'to') {
      funcNameSpecific = `${modelNameLower}ToJsValue`;
    } else {
      funcNameSpecific = `${modelNameLower}FromJsValue`;
    }
  } else if (ffiType === 'msgpack') {
    if (direction === 'encode') {
      funcNameSpecific = `encode_${modelNameLower}`;
    } else {
      funcNameSpecific = `decode_${modelNameLower}`;
    }
  } else {
    throw new Error('Invalid ffi_type');
  }

  if (pkgModule[funcNameSpecific]) {
    return pkgModule[funcNameSpecific];
  }

  if (ffiType === 'msgpack') {
    if (direction === 'encode' && pkgModule.encode_msgpack) {
      return pkgModule.encode_msgpack;
    } else if (direction === 'decode' && pkgModule.decode_msgpack) {
      return pkgModule.decode_msgpack;
    }
  }

  return null;
}

/**
 * Convert a model instance to JSON string
 */
export function modelToJsonStr(modelInstance: any): string {
  const modelName = modelInstance.constructor.name;
  const modelNameSnake = camelToSnakeCase(modelName);
  
  const ffiFunc = getFfiFunc(
    algokit_msgpack,
    modelNameSnake,
    modelNameSnake,
    'to',
    'json'
  );

  if (ffiFunc) {
    return ffiFunc(modelInstance);
  } else {
    throw new FFINotImplementedError(
      `FFI JSON serialization (ToJsValue) not found for ${modelName}`
    );
  }
}

/**
 * Create a model instance from JSON string
 */
export function modelFromJsonStr(modelClass: any, jsonData: string): any {
  const modelName = modelClass.name;
  const modelNameSnake = camelToSnakeCase(modelName);
  
  const ffiFunc = getFfiFunc(
    algokit_msgpack,
    modelNameSnake,
    modelNameSnake,
    'from',
    'json'
  );

  if (ffiFunc) {
    return ffiFunc(jsonData);
  } else {
    throw new FFINotImplementedError(
      `FFI JSON deserialization (FromJsValue) not found for ${modelName}`
    );
  }
}

/**
 * Convert a model instance to MessagePack bytes
 */
export function modelToMsgpack(modelInstance: any): Uint8Array {
  const modelName = modelInstance.constructor.name;
  const modelNameSnake = camelToSnakeCase(modelName);
  
  const ffiFunc = getFfiFunc(
    algokit_msgpack,
    modelNameSnake,
    modelNameSnake,
    'encode',
    'msgpack'
  );

  if (ffiFunc) {
    return ffiFunc(modelInstance);
  } else {
    throw new FFINotImplementedError(
      `FFI MessagePack encoding (encode_*) not found for ${modelName}`
    );
  }
}

/**
 * Create a model instance from MessagePack bytes
 */
export function modelFromMsgpack(modelClass: any, msgpackData: Uint8Array): any {
  const modelName = modelClass.name;
  const modelNameSnake = camelToSnakeCase(modelName);
  
  const ffiFunc = getFfiFunc(
    algokit_msgpack,
    modelNameSnake,
    modelNameSnake,
    'decode',
    'msgpack'
  );

  if (ffiFunc) {
    if (ffiFunc.name === 'decode_msgpack') {
      throw new FFINotImplementedError(
        `Generic FFI MessagePack decoding for ${modelName} is complex from TypeScript helper; prefer specific decode functions.`
      );
    }
    return ffiFunc(msgpackData);
  } else {
    throw new FFINotImplementedError(
      `FFI MessagePack decoding (decode_*) not found for ${modelName}`
    );
  }
}

/**
 * Re-export all models and functions from algokit_msgpack package
 */
export * from '../pkg'; 
