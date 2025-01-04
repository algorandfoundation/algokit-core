import { KeyPairRecord } from "./types";
import {getAddress} from "./wallet";

/**
 * Wrapper around IDBRequest to return a promise
 * @param request
 */
export function asyncRequest<T>(request: IDBRequest<T>){
    return new Promise<T>((resolve, reject) => {
        request.onsuccess = function () {
            resolve(request.result)
        }
        request.onerror = function () {
            reject(request.error)
        }
    })
}

/**
 * Save a key pair to the database
 * @param keyPair
 */
export async function saveKeyPair(keyPair: CryptoKeyPair){
    const address =  await getAddress(keyPair)
    const store = await open()
    return asyncRequest<IDBValidKey>(store.put({keyPair, id: address}))
}

/**
 * Get a key pair from the database
 * @param address
 */
export async function getKeyPair(address: IDBValidKey | IDBKeyRange){
    const store = await open()
    return asyncRequest<KeyPairRecord>(store.get(address))
}

/**
 * Open the database and return the object store
 * @param dbName
 * @param storeName
 */
export async function open(dbName = 'Algorand', storeName = "KeyStore") {
    return new Promise<IDBObjectStore>((resolve, reject) => {
        if(typeof window === 'undefined') return reject(new Error('window is undefined'))
        const dbOpenDBRequest = indexedDB.open(dbName, 1);
        dbOpenDBRequest.onupgradeneeded = function() {
            dbOpenDBRequest.result.createObjectStore(storeName, {keyPath: "id"});
        };
        dbOpenDBRequest.onsuccess = function() {
            const db = dbOpenDBRequest.result;
            const dbTransaction = db.transaction(storeName, "readwrite");
            resolve(dbTransaction.objectStore(storeName))
            dbTransaction.oncomplete = function() {
                db.close();
            };
        }
        dbOpenDBRequest.onerror = function() {
            reject(dbOpenDBRequest.error)
        }
    })
}