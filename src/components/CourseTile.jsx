import Styles from "./styles/courses.module.css";

function CourseTile({ course_information }) {
  console.log(course_information);

  return (
    <a className={Styles.courseTile}>
      <h1>{course_information.course_id}</h1>
      <h3>{course_information.course_name}</h3>
      <p>{course_information.description}</p>
    </a>
  );
}

export default CourseTile;
