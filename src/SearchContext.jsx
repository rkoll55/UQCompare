import React, { createContext, useState, useEffect, useCallback } from "react";
import Papa from "papaparse";

export const SearchContext = createContext();

export const SearchProvider = ({ children }) => {
  const [search, setSearch] = useState("");
  const [courses, setCourses] = useState([]);
  const [filteredCourses, setFilteredCourses] = useState([]);

  useEffect(() => {
    const loadCourses = async () => {
      try {
        const response = await fetch('./course_ids.csv');
        if (!response.ok) {
          throw new Error('Failed to fetch course data');
        }
        const reader = response.body.getReader();
        const result = await reader.read();
        const decoder = new TextDecoder('utf-8');
        const csv = decoder.decode(result.value);
        const parsedData = Papa.parse(csv, { header: true });
        setCourses(parsedData.data);
      } catch (error) {
        console.error("Error loading course data:", error);
      }
    };

    loadCourses();
  }, []);

  const handleSearch = useCallback((query) => {
    if (!query) {
      setFilteredCourses([]);
      return;
    }

    const filtered = courses.filter(course =>
      course.course_id.toLowerCase().includes(query.toLowerCase())
    );
    setFilteredCourses(filtered);
  }, [courses]);

  return (
    <SearchContext.Provider value={{ search, setSearch, filteredCourses, handleSearch }}>
      {children}
    </SearchContext.Provider>
  );
}
