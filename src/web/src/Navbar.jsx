import React from "react";
import { Link } from "react-router-dom";
import salty from "./images/salty.png";
import rec from "./images/rec.png";
import play from "./images/play.svg";
import options from "./images/options.png";

const Navbar = () => {
  return (
    <nav>
      <div>
        <img src={salty} alt="" id="salty-logo" />
        <div className="logo-seperator"></div>
      </div>
      <div>
        <Link to="/record">
          <img src={rec} alt="" />
        </Link>
      </div>
      <div>
        <Link to="/replays">
          <img src={play} alt="" />
        </Link>
      </div>
      <div>
        <img src={options} alt="" id="options" />
      </div>
    </nav>
  );
};

export default Navbar;
