import { useState } from "react";

const styles = {
  search: {
    fontSize: "1rem",
    padding: "10px",
    borderRadius: "15px",
    border: "2px solid #5B247C",
    width: "65%",
    height: "2rem",
    marginLeft: "auto",
    marginRight: "auto",
    marginTop: "2rem",
  },
};

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
      style={styles.search}
    />
  );
}

export default Search;
