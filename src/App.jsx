import { useState } from "react";
import Header from "./components/Header.jsx";
import Poster from "./components/Poster.jsx";
import Courses from "./components/Courses.jsx";
import Footer from "./components/Footer.jsx";

function App() {
  return (
    <>
      <Header />
      <Poster />
      <Courses />
      <Footer />
    </>
  );
}

export default App;
