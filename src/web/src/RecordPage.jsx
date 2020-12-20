import React from "react";

const RecordPage = () => {
  const timerValue = "00:00:00";
  return (
    <section className="record-page-container">
      <div className="record-info">
        <h1 className="page-title">Record Replay</h1>
        <h2>{timerValue}</h2>
        <button type="button">Discard</button>
      </div>
      <ReplayHistory />
    </section>
  );
};

const ReplayHistory = () => {
  return (
    <section className="replay-history">
      <div id="replay-history-line">
        <h3>RECORD HISTORY</h3>
      </div>
      <div className="history-replays"></div>
    </section>
  );
};

const Replay = (props) => {};

export default RecordPage;
