import { Accessor } from "solid-js";
import { IconTypes } from "solid-icons";
import styles from "./boxicon.module.scss";

type BoxIconProps = {
  size: number;
  icon: IconTypes;
  disabled?: Accessor<boolean>;
  onClick?: () => void;
};

export default function BoxIcon({ size, icon, disabled, onClick }: BoxIconProps) {
  return (
    <div
      style={{ width: `${size}px`, height: `${size}px` }}
      class={styles.boxicon}
      classList={{ [styles.disabled]: disabled?.() ?? false }}
      onClick={() => !disabled?.() && onClick?.()}
    >
      {icon({ size: size - 12 })}
    </div>
  );
}
