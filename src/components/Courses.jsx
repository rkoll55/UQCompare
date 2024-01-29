import Search from "./Search";
import Styles from "./styles/courses.module.css";

function Courses() {

  return (
    <section className={Styles.courseSection}>
      <Search />
      <h1 className={Styles.coursesHeader}>Featured Courses...</h1>
    </section>
  );
}

export default Courses;
