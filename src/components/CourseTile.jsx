import Styles from "./styles/courses.module.css";
import { Link } from "react-router-dom";

function CourseTile({ course_information }) {
  return (
    <Link
      className={Styles.courseTile}
      to={"/courses/" + course_information.course_id.toLowerCase()}
    >
      <h1>{course_information.course_id}</h1>
      <h3>{course_information.course_name}</h3>
      <p>{course_information.description}</p>
    </Link>
  );
}

export default CourseTile;
