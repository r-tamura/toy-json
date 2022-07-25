import * as React from "react";
import * as ReactDOM from "react-dom/client";
import * as toyJson from "toy-json-wasm";
interface Sample {
  name: string;
  json: string;
}
interface Debounced {
  (): void;
  cancel: () => void;
}

function debounce(f: Function, delay: number): Debounced {
  let timeout: number | null = null;
  const debounsed = () => {
    if (timeout !== null) {
      window.clearInterval(timeout);
    }
    window.setTimeout(f, delay);
  };
  debounsed.cancel = () => {
    if (timeout !== null) {
      window.clearInterval(timeout);
    }
  };
  return debounsed;
}

function useDebounce(f: Function, delay: number, deps: React.DependencyList) {
  React.useEffect(() => {
    const timeout = window.setTimeout(() => f(), delay);
    return () => {
      window.clearTimeout(timeout);
    };
  }, [delay, ...deps]);
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
    if (sample === "custom") {
      return;
    }
    setInput(SAMPLES[sample].json);
  }, [sample]);

  function handleSampleChange(e: React.ChangeEvent<HTMLSelectElement>) {
    setSample(e.target.value);
  }

  function handleInputChange(e: React.ChangeEvent<HTMLTextAreaElement>) {
    e.preventDefault();
    e.stopPropagation();
    setInput(e.target.value);
    setSample("custom");
  }

  useDebounce(
    () => {
      try {
        if (input === "") {
          return "";
        }
        setOutput(toyJson.format(input, 2));
      } catch (err) {
        setOutput(`Failed to parse JSON:\n ${err}`);
        // throw err;
      }
    },
    1000,
    [input, setOutput]
  );

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    e.stopPropagation();
  }

  return (
    <main style={{ padding: "0 2rem" }}>
      <h1>toy-json-wasm demo</h1>
      <form onSubmit={handleSubmit}>
        <section style={{ display: "flex", gap: "1rem" }}>
          <div>
            <label>
              サンプル:
              <select value={sample} onChange={handleSampleChange}>
                <option value="custom" disabled>
                  custom
                </option>
                {Object.values(SAMPLES).map(({ name }) => (
                  <option key={name} value={name}>
                    {name}
                  </option>
                ))}
              </select>
            </label>
          </div>
        </section>

        <div
          style={{
            display: "grid",
            gridTemplateColumns:
              "minmax(min(100%, 200px), 1fr) minmax(min(100%, 200px), 1fr)",
            margin: "0 auto",
            minHeight: "40rem",
            gap: "1rem",
          }}
        >
          <textarea
            value={input}
            onChange={handleInputChange}
            style={{
              resize: "horizontal",
            }}
          />
          <div>
            <pre
              style={{
                height: "100%",
                margin: "0",
                width: "100%",
                background: "#EFEFEF",
                whiteSpace: "pre-wrap",
              }}
            >
              {output}
            </pre>
          </div>
        </div>
      </form>
    </main>
  );
}

const $root = document.getElementById("root");
if ($root === null) {
  throw Error("react root element doesn't exist");
}

const root = ReactDOM.createRoot($root);
root.render(<App />);
