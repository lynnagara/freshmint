import React from "react";
import ReactDOM from "react-dom";
import styled from "styled-components";

import Client from "./api";

let client = Client();

const WIDTH = 500;
const HEIGHT = 500;

function App() {
  const canvasRef = React.useRef(null);

  return (
    <div>
      <div>Hi, welcome.</div>
      <div>
        <button onClick={echo}>Call echo</button>
      </div>
    </div>
  );
}

const Container = styled.div`
  border: 1px solid black;
  width: ${(props: { width: number; height: number }) => props.width}px;
  height: ${(props: { width: number; height: number }) => props.height}px;
`;

function echo() {
  client.echo("test");
}

ReactDOM.render(<App />, document.getElementById("root"));
