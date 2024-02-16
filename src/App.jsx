import { SearchProvider } from "./SearchContext.jsx";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import MainPage from "./pages/MainPage.jsx";
import RootLayout from "./pages/Root.jsx";
import ErrorPage from "./pages/ErrorPage.jsx";
import CoursePage from "./pages/CoursePage.jsx";

const router = createBrowserRouter([
  {
    path: "/",
    element: <RootLayout />,
    errorElement: <ErrorPage />,
    children: [
      { path: "/", element: <MainPage /> },
      { path: "/courses/:courseId", element: <CoursePage /> },
    ],
  },
]);

function App() {
  return (
    <>
      <SearchProvider>
        <RouterProvider router={router} />
      </SearchProvider>
    </>
  );
}

export default App;
