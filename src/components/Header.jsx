import styles from "./styles/header.module.css";
import { useContext } from "react";
import { SearchContext } from "../SearchContext";
import { Link } from "react-router-dom";
import SearchModal from "./SearchModal.jsx";

export default function Header() {
  const { search, setSearch } = useContext(SearchContext);

  const handleSetSearch = () => {
    setSearch(!search);
  };

  return (
    <header className={styles.header}>
      <SearchModal isOpen={Boolean(search)} closeModal={() => setSearch("")} />
      <Link
        to="/"
        className={styles.mainHeaderContainer}
        style={{ textDecoration: "none", color: "inherit", cursor: "auto" }}
      >
        <h1 className={styles.mainHeader}>UQCOMPARE</h1>
      </Link>
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
