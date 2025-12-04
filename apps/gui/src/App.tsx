import Titlebar from "./components/shared/Titlebar";
import Sidebar from "./components/shared/Sidebar";

import "./App.css";
import Explorer from "./views/Explorer";

function App() {
	return (
		<>
			<Titlebar />
			<main id="main">
				<Sidebar />
				<Explorer />
			</main>
		</>
	);
}

export default App;
