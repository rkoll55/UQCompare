import styles from "./styles/header.module.css";
import { useContext } from "react";
import { SearchContext } from "../SearchContext";

export default function Header() {
  const { search, setSearch } = useContext(SearchContext);

  const handleSetSearch = () => {
    setSearch(!search);
  };

  return (
    <header className={styles.header}>
      <a
        className={styles.mainHeaderContainer}
        href="http://127.0.0.1:5174/"
        style={{ textDecoration: "none", color: "inherit", cursor: "auto" }}
      >
        <h1 className={styles.mainHeader}>UQCOMPARE</h1>
      </a>
      <div className={styles.buttonContainer}>
        <a href="https://uqfinal.com/" className={`${styles.siteLink}`}>
          Exam Mark Calculator
        </a>
        <a href="https://www.uqplanner.app/" className={`${styles.siteLink}`}>
          Plan Your Timetable
        </a>
        <button onClick={handleSetSearch} className={styles.mainButton}>
          Find A Course!
        </button>
      </div>
    </header>
  );
}
