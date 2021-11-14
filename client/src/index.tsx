import React from "react";
import ReactDOM from "react-dom";
import styled from "styled-components";

import Client from "./api";

let client = Client();

const WIDTH = 500;
const HEIGHT = 500;

function App() {
  const canvasRef = React.useRef<HTMLCanvasElement | null>(null);


  function upload(): void {
    if (canvasRef.current === null) {
      return;
    }

    canvasRef.current.toBlob(function(blob: Blob | null) {
      if (blob !== null) {
        client.upload(blob);
      }
    });
  }

  return (
    <div>
      <div>Hi, welcome.</div>
      <div>
        <Container width={WIDTH} height={HEIGHT}>
          <canvas ref={canvasRef} width={WIDTH} height={HEIGHT}></canvas>
        </Container>
      </div>
      <div>
        <button onClick={upload}>Upload</button>
      </div>
    </div>
  );
}

const Container = styled.div`
  border: 1px solid black;
  width: ${(props: { width: number; height: number }) => props.width}px;
  height: ${(props: { width: number; height: number }) => props.height}px;
`;


ReactDOM.render(<App />, document.getElementById("root"));
