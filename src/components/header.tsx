import { AiOutlineFileAdd, AiOutlineCloudDownload, AiOutlineDelete, AiOutlineEdit, AiOutlineSetting } from "solid-icons/ai";
import { Accessor } from "solid-js";
import BoxIcon from "./boxicon";
import styles from "./header.module.scss";

type HeaderProps = {
  addFiles: () => void;
  query: Accessor<string>;
  setQuery: (query: string) => void;
};

export default function Header(props: HeaderProps) {
  return (
    <div class={styles.header}>
      <BoxIcon size={36} icon={AiOutlineFileAdd} onClick={props.addFiles} />
      <div class={styles.search}>
        <input type="text" placeholder="Search..." value={props.query()} onInput={e => props.setQuery(e.target.value)} />
      </div>
      <BoxIcon disabled={() => true} size={36} icon={AiOutlineCloudDownload} />
      <BoxIcon disabled={() => true} size={36} icon={AiOutlineEdit} />
      <BoxIcon disabled={() => true} size={36} icon={AiOutlineDelete} />
      <div class={styles.separator} />
      <BoxIcon size={36} icon={AiOutlineSetting} />
    </div>
  );
}