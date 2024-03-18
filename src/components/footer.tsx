import styles from "./footer.module.scss";

export default function Footer() {
  return (
    <div class={styles.footer}>
      <p class={styles.text}></p>
      <p class={styles.subtext}></p>
      <div class={styles.progress}>
        <div class={styles.bar} style={{ width: "0%" }} />
      </div>
    </div>
  );
}