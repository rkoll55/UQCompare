import { useState } from "react";
import Styles from "./styles/courses.module.css";

function Search() {
  const [value, setValue] = useState("");

  const handleChange = (event) => {
    setValue(event.target.value);
  };

  return (
    <input
      type="text"
      value={value}
      onChange={handleChange}
      placeholder="Search for a Course..."
      className={Styles.search}
    />
  );
}

export default Search;
