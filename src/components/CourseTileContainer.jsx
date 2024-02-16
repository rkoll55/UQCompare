import { useEffect, useState } from "react";
import Search from "./Search";
import Styles from "./styles/courses.module.css";
import CourseTile from "./CourseTile";

function CourseTileContainer() {
  const [courses, setCourses] = useState();

  const fetchData = async () => {
    try {
      const response = await fetch("http://127.0.0.1:8000/api/top/6");
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
      <div className={Styles.coursesBlock}>
        {courses ? (
          courses.map((course) => (
            <CourseTile key={course.course_id} course_information={course} />
          ))
        ) : (
          <h1>Loading...</h1>
        )}
      </div>
    </section>
  );
}

export default CourseTileContainer;
