import styles from "./styles/poster.module.css";
import poster from "../assets/poster-image.jpg";

function Poster() {
  return (
    <>
            <div className={styles.square} />
      <section className={styles.poster}>
        <div className={styles.posterHeading}>
          <h1>Engage, Review and Discuss Your UQ Courses</h1>
          <hr className={styles.lineBreak} />
          <p className={styles.posterInfo}>
            UQCompare provides a comprehensive platform where you can
            effortlessly access essential details about your courses, share
            insightful reviews, and discuss them with your fellow UQ students.
          </p>
        </div>
        <img src={poster} className={styles.posterImage} />
      </section>
    </>
  );
}

export default Poster;
