import { useParams } from "react-router-dom";
import { useState, useEffect } from "react";
import ECPSummary from "../components/ECPSummary";
import Styles from "./styles/coursepage.module.css";
import QnA from "../components/QnA";
import Review from "../components/Review";

function CoursePage() {
  const params = useParams();
  const courseId = `${import.meta.env.VITE_API_URL}/api/get/${params.courseId}`;
  const [courseInfo, setCourseInfo] = useState();

  const fetchData = async () => {
    try {
      const response = await fetch(courseId);
      if (!response.ok) {
        throw new Error("Network response was not ok");
      }
      const result = await response.json();
      setCourseInfo(result);
    } catch (error) {
      console.log(error.message);
      setCourseInfo("error");
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  return (
    <section className={Styles.ecpSummary}>
      {courseInfo ? (
        <>
          <ECPSummary courseDetails={courseInfo} />
          <QnA courseId={params.courseId}/>
          <Review courseId={params.courseId} />
        </>
      ) : (
        <h1 style={{ textAlign: "center" }}>Loading...</h1>
      )}
    </section>
  );
}

export default CoursePage;
