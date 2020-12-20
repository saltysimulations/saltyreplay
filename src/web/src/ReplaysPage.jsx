import React, { useState } from "react";
import play from "./images/play.png";
import pause from "./images/pause.svg";
import replays from "./replays.json";

const ReplaysPage = () => {
  return (
    <section className="replays-page-container">
      <div className="replays-title">
        <h1>Replays</h1>
      </div>
      <div className="replays">
        {replays.replays.map((replay) => {
          let imgSize = "small-replay-play-button";
          if (replay.id === 1 || replay.id === 4) {
            imgSize = "replay-play-button";
          }
          return <Replay data={replay} imgSize={imgSize} />;
        })}
      </div>
    </section>
  );
};

const Replay = (props) => {
  const { timestamp, length, area, aircraft } = props.data;
  const [playing, setPlaying] = useState(play);

  const handlePlay = () => {
    if (playing === play) {
      setPlaying(pause);
    } else {
      setPlaying(play);
    }
  };

  return (
    <div className="replay">
      <h3>{timestamp}</h3>
      <div className="replay-info">
        <h4>
          Length <span className="replay-info-bold">{length}</span>
        </h4>
        <h4>
          Area <span className="replay-info-bold">{area}</span>
        </h4>
        <h4>
          A/C <span className="replay-info-bold">{aircraft}</span>
        </h4>
      </div>
      <div className={props.imgSize} onClick={handlePlay}>
        <Pause fill="#404040"></Pause>
      </div>
    </div>
  );
};

const Pause = (props) => {
  return (
<svg version="1.1" id="Layer_1" xmlns="http://www.w3.org/2000/svg" x="0px" y="0px"
	 viewBox="0 0 4096 4096">
<g>
	<rect x="758" y="422" className="st0" width="928" height="3252"/>
</g>
<g>
	<rect x="2410" y="422" className="st0" width="928" height="3252"/>
</g>
</svg>
  );
}

export default ReplaysPage;
