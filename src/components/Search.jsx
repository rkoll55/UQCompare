import React, { forwardRef } from "react";
import Styles from "./styles/courses.module.css";
import { useContext } from "react";
import { SearchContext } from "../SearchContext";

const Search = forwardRef((props, ref) => {
  const { search, setSearch } = useContext(SearchContext);

  const handleSetSearch = () => {
    if (!search) {
      setSearch(!search);
    }
  };

  return (
    <input
      ref={ref}
      type="text"
      placeholder="Search for a Course..."
      onClick={handleSetSearch}
      className={Styles.search}
      {...props}
    />
  );
});

export default Search;
