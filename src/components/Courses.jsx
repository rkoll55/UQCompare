import { useEffect, useState } from "react";
import Search from "./Search";
import Styles from "./styles/courses.module.css";

function Courses() {
  const [courses, setCourses] = useState([]);
  const [name, setName] = useState("Nothing");

  const fetchData = async () => {
    try {
      const response = await fetch("http://127.0.0.1:8080/");
      if (!response.ok) {
        throw new Error("Network response was not ok");
      }

      const result = await response.json();
      setName(result);
    } catch (error) {
      setError(error.message);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  return (
    <section className={Styles.courseSection}>
      <Search />
      <h1 className={Styles.coursesHeader}>Featured Courses...</h1>
      <h1>{name}</h1>
      {courses.map((course) => {
        return (
          <div>
            <h2>course.name</h2>
            <h2>course.description</h2>
          </div>
        );
      })}
    </section>
  );
}

export default Courses;
