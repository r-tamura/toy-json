import * as React from "react";
import * as ReactDOM from "react-dom/client";
import * as toyJson from "toy-json-wasm";

function TextArea() {
  return (
    <textarea
      style={{
        resize: "horizontal",
        minWidth: "10rem",
        width: "20rem",
        minHeight: "40rem",
      }}
    ></textarea>
  );
}

function App() {
  const [input, setInput] = React.useState(`{"Hello, Wasm!": true, "list"
      : [1,
        2,

    3], "object": {
            "prop1":
      "日本語", "prop2": "🎯🌀"

  }}`);
  const [output, setOutput] = React.useState("");

  function handleInputChange(e: React.ChangeEvent<HTMLTextAreaElement>) {
    setInput(e.target.value);
  }

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    e.stopPropagation();
    try {
      setOutput(toyJson.format(input, 2));
    } catch (err) {
      setOutput("");
      throw err;
    }
  }

  return (
    <div>
      <h1>Hello, WASM</h1>
      <form onSubmit={handleSubmit}>
        <div
          style={{
            display: "flex",
            alignContent: "center",
            margin: "0 auto",
            width: "900px",
            minHeight: "40rem",
            gap: "3rem",
          }}
        >
          <textarea
            value={input}
            onChange={handleInputChange}
            style={{
              resize: "horizontal",
              minWidth: "20rem",
              width: "20rem",
              minHeight: "40rem",
            }}
          />
          <div>
            <button type="submit">{">"}</button>
          </div>
          <pre style={{ margin: "0", width: "100%", background: "#EFEFEF" }}>
            {output}
          </pre>
        </div>
      </form>
    </div>
  );
}

const $root = document.getElementById("root");
if ($root === null) {
  throw Error("react root element doesn't exist");
}

const root = ReactDOM.createRoot($root);
root.render(<App />);
