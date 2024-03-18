import { Accessor, For, Show } from "solid-js";
import styles from "./body.module.scss";
import { Portal } from "solid-js/web";

type BodyProps = {
  query: Accessor<string>;
  queue: Accessor<IQueuedFile[]>;
  files: Accessor<IFile[]>;
  isHovering: Accessor<boolean>;
};

export default function Body(props: BodyProps) {
  return (
    <div class={styles.body}>
      <For each={props.queue()}>
        {(file) => <QueuedFile file={file} />}
      </For>
      <For each={props.files()}>
        {(file) => <File file={file} />}
      </For>
      <Show when={props.queue().length === 0 && props.files().length === 0}>
        <Fallback searching={props.query().length > 0} />
      </Show>
      <div class={styles.dropzone} classList={{ [styles.hover]: props.isHovering() }} />
    </div>
  );
}

function Fallback({ searching }: { searching: boolean }) {
  return (
    <div class={styles.fallback}>
      <h1>(╯°□°)╯︵ ┻━┻</h1>
      <p>{searching ? "No files match your search" : "Drop files here to upload"}</p>
    </div>
  )
}

function QueuedFile({ file }: { file: IQueuedFile }) {
  return (
    <>
    </>
  );
}

function File({ file }: { file: IFile }) {
  return (
    <>
    </>
  );
}
