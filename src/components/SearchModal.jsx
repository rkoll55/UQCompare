import React, { useEffect, useRef, useContext } from "react";
import ReactDOM from "react-dom";
import { useNavigate } from "react-router-dom";
import Search from "./Search";
import { SearchContext } from "../SearchContext";

const styles = {
  overlay: {
    position: "fixed",
    top: 0,
    left: 0,
    width: "100vw",
    height: "100vh",
    backgroundColor: "rgba(0, 0, 0, 0.7)",
    zIndex: 1,
  },

  dialog: {
    position: "fixed",
    bottom: "10%",
    width: "64%",
    height: "80vh",
    display: "flex",
    flexDirection: "column",
    justifyContent: "center",
    borderRadius: "2rem",
    zIndex: 2,
    border: "4px solid #44619e",
  },

  search: {
    width: "85%",
  },

  results: {
    marginTop: "1rem",
    width: "100%",
    maxHeight: "60vh",
    overflowY: "auto",
    backgroundColor: "#fff",
    borderRadius: "1rem",
    padding: "1rem",
  },

  resultItem: {
    padding: "0.5rem",
    borderBottom: "1px solid #ddd",
    cursor: "pointer",
  },
};

const SearchModal = ({ isOpen, closeModal }) => {
  const modalRef = useRef(null);
  const searchRef = useRef(null);
  const { filteredCourses } = useContext(SearchContext);
  const navigate = useNavigate();

  useEffect(() => {
    if (isOpen) {
      searchRef.current.focus();
    }
  }, [isOpen]);

  useEffect(() => {
    const handleOutsideClick = (event) => {
      if (modalRef.current && !modalRef.current.contains(event.target)) {
        closeModal();
      }
    };

    const handleKeyPress = (event) => {
      if (event.key === "Escape") {
        closeModal();
      }
    };

    document.addEventListener("mousedown", handleOutsideClick);
    document.addEventListener("keydown", handleKeyPress);

    return () => {
      document.removeEventListener("mousedown", handleOutsideClick);
      document.removeEventListener("keydown", handleKeyPress);
    };
  }, [closeModal]);

  const handleResultClick = (courseId) => {
    navigate(`/courses/${courseId}`);
    closeModal();
  };

  if (!isOpen) {
    return null;
  }

  return ReactDOM.createPortal(
    <div style={styles.overlay}>
      <dialog open className="modal" ref={modalRef} style={styles.dialog}>
        <Search style={styles.search} ref={searchRef} />
        <div style={styles.results}>
          {filteredCourses.map((course, index) => (
            <div
              key={index}
              style={styles.resultItem}
              onClick={() => handleResultClick(course.course_id)}
            >
              {course.course_id}
            </div>
          ))}
        </div>
      </dialog>
    </div>,
    document.getElementById("root")
  );
};

export default SearchModal;
