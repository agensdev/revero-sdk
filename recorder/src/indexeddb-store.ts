import type { DurableRecorderState, RecorderStore } from "./core"

const STATE_KEY = "current"

interface StateRow {
  key: typeof STATE_KEY
  value: DurableRecorderState
}

interface ChunkRow {
  id?: number
  sessionId: string
  chunk: Blob
}

export interface IndexedDbRecorderStoreOptions {
  databaseName?: string
}

export class IndexedDbRecorderStore implements RecorderStore {
  readonly #databaseName: string
  #database?: Promise<IDBDatabase>

  constructor(options: IndexedDbRecorderStoreOptions = {}) {
    this.#databaseName = options.databaseName ?? "revero-recorder"
  }

  async loadState(): Promise<DurableRecorderState | null> {
    const row = await this.#request<StateRow | undefined>("state", "readonly", (store) =>
      store.get(STATE_KEY)
    )
    return row?.value ?? null
  }

  async saveState(state: DurableRecorderState): Promise<void> {
    await this.#request("state", "readwrite", (store) =>
      store.put({ key: STATE_KEY, value: state } satisfies StateRow)
    )
  }

  async appendChunk(sessionId: string, chunk: Blob): Promise<void> {
    await this.#request("chunks", "readwrite", (store) =>
      store.add({ sessionId, chunk } satisfies ChunkRow)
    )
  }

  async loadChunks(sessionId: string): Promise<Blob[]> {
    const database = await this.#open()
    return new Promise((resolve, reject) => {
      const transaction = database.transaction("chunks", "readonly")
      const index = transaction.objectStore("chunks").index("sessionId")
      const request = index.getAll(IDBKeyRange.only(sessionId))
      request.onerror = () => reject(request.error)
      request.onsuccess = () => resolve((request.result as ChunkRow[]).map(({ chunk }) => chunk))
    })
  }

  async clearChunks(sessionId: string): Promise<void> {
    const database = await this.#open()
    await new Promise<void>((resolve, reject) => {
      const transaction = database.transaction("chunks", "readwrite")
      const store = transaction.objectStore("chunks")
      const request = store.index("sessionId").openKeyCursor(IDBKeyRange.only(sessionId))
      request.onerror = () => reject(request.error)
      request.onsuccess = () => {
        const cursor = request.result
        if (!cursor) return
        store.delete(cursor.primaryKey)
        cursor.continue()
      }
      transaction.oncomplete = () => resolve()
      transaction.onerror = () => reject(transaction.error)
      transaction.onabort = () => reject(transaction.error)
    })
  }

  async clear(): Promise<void> {
    const database = await this.#open()
    await new Promise<void>((resolve, reject) => {
      const transaction = database.transaction(["state", "chunks"], "readwrite")
      transaction.objectStore("state").clear()
      transaction.objectStore("chunks").clear()
      transaction.oncomplete = () => resolve()
      transaction.onerror = () => reject(transaction.error)
      transaction.onabort = () => reject(transaction.error)
    })
  }

  async #request<T = undefined>(
    storeName: "state" | "chunks",
    mode: IDBTransactionMode,
    operation: (store: IDBObjectStore) => IDBRequest
  ): Promise<T> {
    const database = await this.#open()
    return new Promise((resolve, reject) => {
      const request = operation(database.transaction(storeName, mode).objectStore(storeName))
      request.onerror = () => reject(request.error)
      request.onsuccess = () => resolve(request.result as T)
    })
  }

  #open(): Promise<IDBDatabase> {
    if (!this.#database) {
      this.#database = new Promise((resolve, reject) => {
        const request = indexedDB.open(this.#databaseName, 1)
        request.onerror = () => reject(request.error)
        request.onupgradeneeded = () => {
          const database = request.result
          if (!database.objectStoreNames.contains("state")) {
            database.createObjectStore("state", { keyPath: "key" })
          }
          if (!database.objectStoreNames.contains("chunks")) {
            const chunks = database.createObjectStore("chunks", {
              keyPath: "id",
              autoIncrement: true,
            })
            chunks.createIndex("sessionId", "sessionId")
          }
        }
        request.onsuccess = () => resolve(request.result)
      })
    }
    return this.#database
  }
}
