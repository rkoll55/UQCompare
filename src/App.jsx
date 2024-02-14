import { SearchProvider } from "./SearchContext.jsx";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import MainPage from "./pages/MainPage.jsx";
import RootLayout from "./pages/Root.jsx";

const router = createBrowserRouter([
  {
    path: "/",
    element: <RootLayout />,
    children: [{ path: "/", element: <MainPage /> }],
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
