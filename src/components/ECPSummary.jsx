import Styles from "./styles/courses.module.css";

function ECPSummary({ courseDetails }) {
  const title = {
    fontSize: "1.5rem",
  };

  const assessmentList = courseDetails.assesments ? courseDetails.assesments.map(assesment => (
    <li key={assesment.name}>{assesment.name}: {assesment.weight}%</li>
  )): <p>None</p> ;



  return (
    <div className={Styles.ecpDetails}>
      <div style={{ gridArea: "header" }}>
        <h1>{courseDetails.course_id.toUpperCase()}</h1>
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
      <div style={{ gridArea: "assesment", textAlign: "center" }}>
        <h3 style={title}>Assesments</h3>
        <ul style={{listStyleType: "none"}}>
        {assessmentList}
        </ul>
      </div>
    </div>
  );
}

export default ECPSummary;
