import React, { useEffect, useRef } from "react";
import ReactDOM from "react-dom";
import Search from "./Search";

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
    flexDirection: "colum",
    justtifyContent: "center",
    borderRadius: "2rem",
    zIndex: 2,
    border: "4px solid #51247a",
  },

  search: {
    width: "85%",
  },
};

const SearchModal = ({ isOpen, closeModal }) => {
  const modalRef = useRef(null);
  const searchRef = useRef(null);

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

  if (!isOpen) {
    return null;
  }

  return ReactDOM.createPortal(
    <div style={styles.overlay}>
      <dialog open className="modal" ref={modalRef} style={styles.dialog}>
        <Search style={styles.search} ref={searchRef} />
      </dialog>
    </div>,
    document.getElementById("root")
  );
};

export default SearchModal;
