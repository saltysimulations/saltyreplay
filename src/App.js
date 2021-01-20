import Navbar from "./Navbar";
import Playbar from "./Playbar";
import RecordPage from "./RecordPage";
import ReplaysPage from "./ReplaysPage";
import play from "./images/play.svg";
import rec from "./images/rec.png";
import pause from "./images/pause.svg";
import stop from "./images/stop.svg";
import { BrowserRouter, Switch, Route } from "react-router-dom";

function App() {
  return (
    <BrowserRouter>
      <Switch>
        <Route exact path="/record">
          <RecordPage />
        </Route>
        <Route exact path="/replays">
          <ReplaysPage />
        </Route>
      </Switch>
      <Navbar />
      <Playbar img={rec} />
    </BrowserRouter>
  );
}

export default App;
