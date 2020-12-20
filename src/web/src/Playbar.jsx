import React, { useState } from "react";
import rec from "./images/rec.png";
import stop from "./images/stop.png";

const Playbar = (props) => {
  const [record, setRecord] = useState(rec);

  const handleRecord = () => {
    if (record === rec) {
      setRecord(stop);
      window.external.invoke("rec_start");
    } else {
      setRecord(rec);
      window.external.invoke("rec_stop");
    }
  };

  return (
    <div className="playbar-container">
      <div>
        <img src={record} alt="" onClick={handleRecord} />
      </div>
    </div>
  );
};

export default Playbar;
