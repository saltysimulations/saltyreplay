import React, { useState, useEffect } from "react";
import axios from "axios";

const ReplaysPage = () => {
  const [replays, setReplays] = useState({ data: [] });

  useEffect(() => {
    const fetchData = async () => {
      const result = await axios("replaydata");
      setReplays(result.data);
    };
    fetchData();
  }, []);

  return (
    <section className="replays-page-container">
      <div className="replays-title">
        <h1>Replays</h1>
      </div>
      <div className="replays">
        {replays.data.map((replay) => {
          let imgSize = "small-replay-play-button";
          // Temporary solution, just for setting the proper image size for the big tiles.
          if (replay.id === 0 || replay.id === 3) {
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
  const [playing, setPlaying] = useState(false);

  const handlePlay = () => {
    console.log(playing);
    if (playing) {
      setPlaying(false);
    } else {
      setPlaying(true);
    }
  };

  const evaluateImage = () => {
    return playing ? <Pause /> : <Play />;
  };

  const image = evaluateImage();

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
        {image}
      </div>
    </div>
  );
};

const Pause = (props) => {
  return (
    <svg
      version="1.1"
      id="Layer_1"
      xmlns="http://www.w3.org/2000/svg"
      x="0px"
      y="0px"
      viewBox="0 0 4096 4096"
    >
      <g>
        <rect x="758" y="422" className="st0" width="928" height="3252" />
      </g>
      <g>
        <rect x="2410" y="422" className="st0" width="928" height="3252" />
      </g>
    </svg>
  );
};

const Play = (props) => {
  return (
    <svg
      version="1.1"
      id="play-svg"
      xmlns="http://www.w3.org/2000/svg"
      x="0px"
      y="0px"
      viewBox="0 0 4096 4096"
    >
      <polygon className="st0" points="1223,1822.8 2712,1822.8 1223,841.8 " />
      <polygon className="st0" points="1223,2072 2712,2072 1223,3053 " />
    </svg>
  );
};

export default ReplaysPage;
