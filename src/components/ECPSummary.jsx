import Styles from "./styles/courses.module.css";

function ECPSummary({ courseDetails }) {
  const title = {
    fontSize: "1.5rem",
  };

  return (
    <div className={Styles.ecpDetails}>
      <div style={{ gridArea: "header" }}>
        <h1>{courseDetails.course_id}</h1>
        <h2 className={Styles.ecpTitle}>{courseDetails.course_name}</h2>
      </div>
      <div style={{ gridArea: "description" }}>
        <h3 style={title}>Description</h3>
        <p>{courseDetails.description}</p>
      </div>
      <div style={{ gridArea: "lecturer", textAlign: "center" }}>
        <h3 style={title}>Lecturer</h3>
        <p>{courseDetails.lecturer}</p>
      </div>
      <div style={{ gridArea: "prereq", textAlign: "center" }}>
        <h3 style={title}>Prerequisites</h3>
        <p>
          {courseDetails.prerequisites.length != 0
            ? courseDetails.prerequisites
            : "None"}
        </p>
      </div>
      <dev style={{ gridArea: "assesment", textAlign: "center" }}>
        <h3 style={title}>Assesments</h3>
        <p>Coming Soon!</p>
      </dev>
      <dev style={{ gridArea: "secat" }}>
        <h3 style={title}>Secat</h3>
        <p>Coming Soon!</p>
      </dev>
    </div>
  );
}

export default ECPSummary;
