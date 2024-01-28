import styles from "./styles/header.module.css";

export default function Header() {
  return (
    <header className={styles.header}>
      <h1 className={styles.mainHeader}>UQCOMPARE</h1>
      <div className={styles.buttonContainer}>
        <a href="https://uqfinal.com/" className={`${styles.siteLink}`}>
          Exam Mark Calculator
        </a>
        <a href="https://www.uqplanner.app/" className={`${styles.siteLink}`}>
          Plan Your Timetable
        </a>
        <button
          onClick={() => console.log("hello")}
          className={styles.mainButton}
        >
          Find A Course!
        </button>
      </div>
    </header>
  );
}
