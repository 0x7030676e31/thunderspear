export {}
declare global {
  interface IQueuedFile {
    id: number,
    path: string,
  }

  interface IFile {
    id: number,
    name: string,
    size: number,
    created_at: number,
  }
}
