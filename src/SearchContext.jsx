import React, { useState, createContext } from "react";
import SearchModal from "./components/SearchModal.jsx";

const SearchContext = createContext();

function SearchProvider({ children }) {
  const [search, setSearch] = useState(false);

  const handleToggleSearch = () => {
    setSearch(!search);
  };

  return (
    <SearchContext.Provider value={{ search, setSearch }}>
      <SearchModal isOpen={search} closeModal={handleToggleSearch} />
      {children}
    </SearchContext.Provider>
  );
}

export { SearchProvider, SearchContext };
