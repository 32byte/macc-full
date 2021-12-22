import React, { useState } from "react";
import ReactDOM from "react-dom";

import App from "./App";

const wasm = import("../build/macc_wasm");

wasm.then(m => {
  ReactDOM.render(<App wasm={m}/>, document.getElementById("root"));
});