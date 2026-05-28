import { createBrowserRouter, Navigate, RouterProvider } from "react-router-dom";
import DashboardV2 from "./dashboard/DashboardV2";

const router = createBrowserRouter([
  { path: "/", element: <Navigate to="/v2/dashboard" replace /> },
  { path: "/v2", element: <Navigate to="/v2/dashboard" replace /> },
  { path: "/v2/dashboard", element: <DashboardV2 /> },
]);

export default function AppV2() {
  return (
    <div className="dark">
      <RouterProvider router={router} />
    </div>
  );
}
