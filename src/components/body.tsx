import { Accessor } from "solid-js";

type BodyProps = {
  query: Accessor<string>;
};

export default function Body(props: BodyProps) {
  return (
    <></>
  );
}

function Fallback({ searching }: { searching: boolean }) {
  return (
    <div class="fallback">
      <h1>(╯°□°)╯︵ ┻━┻</h1>
      <p>{searching ? "No files match your search" : "Drop files here to upload"}</p>
    </div>
  )
}