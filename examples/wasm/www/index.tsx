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

function useToggle(initialValue = false) {
  const [on, set] = React.useState(initialValue);

  const toggle = React.useCallback(() => {
    set((prev) => !prev);
  }, [on, set]);

  return [on, toggle, set] as const;
}

const SAMPLES: Record<string, Sample> = {
  シンプルオブジェクト: {
    name: "シンプルオブジェクト",
    json: `{"Image": {"Width":  800,"Height": 600,"Title":  "View from 15th Floor","Thumbnail": {"Url":"http://www.example.com/image/481989943","Height": 125,"Width":  100},"Animated" : false,"IDs": [116, 943, 234, 38793]}}`,
  },
  文字列: {
    name: "文字列",
    json: `"Hello, World!"`,
  },
  数値: {
    name: "数値",
    json: `3.1415`,
  },
  いろいろなデータ型: {
    name: "いろいろなデータ型",
    json: `{
      "number": 42,
"boolean": true, "string":
                "Hello, World!"

                ,"nullType": null,
                "array"
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
  Unicode: {
    name: "Unicode",
    json: String.raw`{"key": "\u30CF\u30ED\u30FC\u30EF\u30FC\u30EB\u30C9\u2600\uFE0F"}`,
  },
};

function App() {
  const [sample, setSample] = React.useState(Object.keys(SAMPLES)[0]);
  const [input, setInput] = React.useState("");
  const [format, toggleFormat] = useToggle(true);

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
    console.debug(e.target.value);
    e.preventDefault();
    e.stopPropagation();
    setInput(e.target.value);
    setSample("custom");
  }

  let output;
  try {
    if (input === "") {
      output = "";
    } else {
      output = toyJson.format(input, format ? 2 : 0);
    }
  } catch (err) {
    output = `Failed to parse JSON:\n ${err}`;
  }

  // useDebounce(
  //   () => {
  //     try {
  //       if (input === "") {
  //         return "";
  //       }
  //       setOutput(toyJson.format(input, format ? 2 : 0));
  //     } catch (err) {
  //       setOutput(`Failed to parse JSON:\n ${err}`);
  //       // throw err;
  //     }
  //   },
  //   1000,
  //   [input, setOutput, format]
  // );

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    e.stopPropagation();
  }

  return (
    <main style={{ padding: "0 2rem" }}>
      <h1>toy-json-wasm demo</h1>
      <form onSubmit={handleSubmit}>
        <section style={{ display: "flex", gap: "1rem", padding: "1rem 0" }}>
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
          <div>
            <label>
              Format:
              <input
                type="checkbox"
                style={{ fontFamily: "monospace;" }}
                checked={format}
                onChange={(e) => toggleFormat()}
              />
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
            onInput={handleInputChange}
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
