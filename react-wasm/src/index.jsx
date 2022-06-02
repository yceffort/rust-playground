import React, { useState } from "react";
import ReactDOM from "react-dom";

const wasm = import("../build/react_wasm");

wasm.then((m) => {
  const App = () => {
    const [name, setName] = useState("");
    const handleChange = (e) => {
      setName(e.target.value);
    };

    const handleClick = () => {
      m.welcome(name);
    };

    const handleBigComputation = () => {
      wasm.big_computation()
    }

    return (
      <>
        <div>
          <h1>Hi there</h1>
          <button onClick={handleBigComputation}>Run Computation</button>
        </div>
        <div>
          <input type="text" onChange={handleChange} />
          <button onClick={handleClick}>Say hello!</button>
        </div>
      </>
    );
  };

  ReactDOM.render(<App />, document.getElementById("root"));
});