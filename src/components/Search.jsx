import React, { forwardRef, useContext, useState } from "react";
import Styles from "./styles/courses.module.css";
import { SearchContext } from "../SearchContext";

const Search = forwardRef((props, ref) => {
  const { search, setSearch, handleSearch } = useContext(SearchContext);
  const [query, setQuery] = useState("");

  const handleSetSearch = () => {
    if (!search) {
      setSearch(true);
    }
  };

  const handleChange = (e) => {
    const value = e.target.value;
    setQuery(value);
    handleSearch(value);
  };

  return (
    <input
      ref={ref}
      type="text"
      placeholder="Search for a Course..."
      value={query}
      onClick={handleSetSearch}
      onChange={handleChange}
      className={Styles.search}
      {...props}
    />
  );
});

export default Search;