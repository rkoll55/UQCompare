import { useState } from "react";
import Header from "./components/Header.jsx";
import Poster from "./components/Poster.jsx";
import Courses from "./components/Courses.jsx";
import Footer from "./components/Footer.jsx";
import SearchModal from "./components/SearchModal.jsx";
import { SearchProvider } from "./SearchContext.jsx";

function App() {
  return (
    <>
      <SearchProvider>
        <Header />
        <Poster />
        <Courses />
        <Footer />
      </SearchProvider>
    </>
  );
}

export default App;
