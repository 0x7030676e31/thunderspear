import { createSignal, onCleanup, onMount } from "solid-js";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import Header from "./components/header";
import Body from "./components/body";
import "./index.scss";

export default function App() {
  const [ dropHover, setDropHover ] = createSignal(false);
  const [ query, setQuery ] = createSignal("");

  let unlisten_drop: UnlistenFn;
  let unlisten_hover: UnlistenFn;
  let unlisten_cancel: UnlistenFn;
  
  onMount(async () => {
    unlisten_drop = await listen<string[]>("tauri://file-drop", async ({ payload }) => {
      setDropHover(false);
    });

    unlisten_hover = await listen("tauri://file-drop-hover", () => {
      setDropHover(true);
    });

    unlisten_cancel = await listen("tauri://file-drop-cancelled", () => {
      setDropHover(false);
    });

    await listen("uploading", ({ payload }) => console.log(payload));
  });

  async function addFiles() {
    const payload = await open({ title: "Thunderspear - Select Files", directory: false, multiple: true });
    if (!payload) return;

    await invoke("upload_files", { files: payload });
  }
  
  onCleanup(() => {
    unlisten_drop();
    unlisten_hover();
    unlisten_cancel();
  });

  return (
    <>
      <Header addFiles={addFiles} query={query} setQuery={setQuery} />
      <Body query={query} />
    </>
  );
}
