import * as React from "react";
import * as ReactDOM from "react-dom/client";
import * as toyJson from "toy-json-wasm";

interface Sample {
  name: string;
  json: string;
}

const SAMPLES: Record<string, Sample> = {
  いろいろなデータ型: {
    name: "いろいろなデータ型",
    json: `{"Hello, Wasm!": true, "list"
      : [1,
        2,

    3], "object": {
            "prop1":
      "日本語", "prop2": "🎯🌀"

  }}`,
  },
  空オブジェクト: {
    name: "空オブジェクト",
    json: "{}",
  },
};

function App() {
  const [sample, setSample] = React.useState(Object.keys(SAMPLES)[0]);
  const [input, setInput] = React.useState("");
  const [output, setOutput] = React.useState("");

  React.useEffect(() => {
    console.debug("sample changed:", sample);
    setInput(SAMPLES[sample].json);
  }, [sample]);

  function handleSampleChange(e: React.ChangeEvent<HTMLSelectElement>) {
    setSample(e.target.value);
  }

  function handleInputChange(e: React.ChangeEvent<HTMLTextAreaElement>) {
    e.preventDefault();
    e.stopPropagation();
    setInput(e.target.value);
  }

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    e.stopPropagation();
    try {
      setOutput(toyJson.format(input, 2));
    } catch (err) {
      setOutput(`Failed to parse JSON:\n ${err}`);
      throw err;
    }
  }

  return (
    <div>
      <h1>toy-json-wasm demo</h1>
      <form onSubmit={handleSubmit}>
        <select value={sample} onChange={handleSampleChange}>
          {Object.values(SAMPLES).map(({ name }) => (
            <option key={name} value={name}>
              {name}
            </option>
          ))}
        </select>
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
