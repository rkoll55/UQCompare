import { useEffect, useState } from "react";
import Search from "./Search";
import Styles from "./styles/courses.module.css";

function CoursesContainer() {
  const [courses, setCourses] = useState();

  const fetchData = async () => {
    try {
      const response = await fetch("http://127.0.0.1:8000/courses/top/5");
      if (!response.ok) {
        throw new Error("Network response was not ok");
      }
      const result = await response.json();
      setCourses(result);
    } catch (error) {
      console.log(error.message);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  return (
    <section className={Styles.courseSection}>
      <Search />
      <h1 className={Styles.coursesHeader}>Featured Courses...</h1>
      {courses ? (
        courses.map((course) => <h2>{course.course_id}</h2>)
      ) : (
        <h1>Loading...</h1>
      )}
    </section>
  );
}

export default CoursesContainer;
