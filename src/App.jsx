import { useState } from "react";
import Header from "./components/Header.jsx";
import Poster from "./components/Poster.jsx";
import CoursesContainer from "./components/CoursesContainer.jsx";
import Footer from "./components/Footer.jsx";
import SearchModal from "./components/SearchModal.jsx";
import { SearchProvider } from "./SearchContext.jsx";

function App() {
  return (
    <>
      <SearchProvider>
        <Header />
        <Poster />
        <CoursesContainer />
        <Footer />
      </SearchProvider>
    </>
  );
}

export default App;
