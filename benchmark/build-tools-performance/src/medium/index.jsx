import React from "react";
import ReactDom from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import App1 from "./f0";

const App2 = React.lazy(() => import("./f1"));
const App3 = React.lazy(() => import("./f2")); 
const App4 = React.lazy(() => import("./f3"));
const App5 = React.lazy(() => import("./f4"));
const App6 = React.lazy(() => import("./f5"));
const App7 = React.lazy(() => import("./f6"));
const App8 = React.lazy(() => import("./f7"));

ReactDom.createRoot(document.getElementById("root")).render(
	<React.StrictMode>
		<BrowserRouter>
			<App1 />
			<React.Suspense fallback={<div>Loading...</div>}>
				<Routes>
					<Route path="/f1" element={<App2 />} />
					<Route path="/f2" element={<App3 />} />
					<Route path="/f3" element={<App4 />} />
					<Route path="/f4" element={<App5 />} />
					<Route path="/f5" element={<App6 />} />
					<Route path="/f6" element={<App7 />} />
					<Route path="/f7" element={<App8 />} />
				</Routes>
			</React.Suspense>
		</BrowserRouter>
	</React.StrictMode>
);
