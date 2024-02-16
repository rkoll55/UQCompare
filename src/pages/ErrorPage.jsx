import Footer from "../components/Footer";
import Header from "../components/Header";
import styles from "./styles/error.module.css";
import { Link } from "react-router-dom";

function ErrorPage() {
  return (
    <>
      <Header />
      <div className={styles.errorBody}>
        <h1>Uh Oh...</h1>
        <h3>We couldn't find the page you're looking for.</h3>
        <Link to={"/"}>
          <button className={styles.errorButton}>Return Home</button>
        </Link>
      </div>
      <Footer />
    </>
  );
}

export default ErrorPage;
