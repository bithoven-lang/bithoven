// App.tsx
import { useState, useCallback } from "react";
import * as bithoven from "bithoven";
import CodeMirror from "@uiw/react-codemirror";
import { customKeywordHighlighting } from "./pretty";
import "./App.css";

const defaultCode = `pragma bithoven version 0.0.1;
pragma bithoven target segwit;

(condition: bool, sig_alice: signature)
(condition: bool, preimage: string, sig_bob: signature)
{
    // If want to spend if branch, condition witness item should be true.
    if condition {
        // Relative locktime for 1000 block confirmation.
        older 1000;
        // If locktime satisfied, alice can redeem by providing signature.
        return checksig (sig_alice, "0245a6b3f8eeab8e88501a9a25391318dce9bf35e24c377ee82799543606bf5212");
    } else {
        // Bob needs to provide secret preimage to unlock hash lock.
        verify sha256 sha256 preimage == "53de742e2e323e3290234052a702458589c30d2c813bf9f866bef1b651c4e45f";
        // If hashlock satisfied, bob can redeem by providing signature.
        return checksig (sig_bob, "0345a6b3f8eeab8e88501a9a25391318dce9bf35e24c377ee82799543606bf5212");
    }
}
`;

function App() {
  const [code, setCode] = useState<string>(defaultCode);
  const [output, setOutput] = useState<string>(
    'Click "Compile & Run" to execute the code.'
  );
  const [isLoading, setIsLoading] = useState<boolean>(false);

  const handleCompile = useCallback(async () => {
    setIsLoading(true);
    setOutput("Compiling...");

    try {
      const compiled = bithoven.compile_program(code).to_object();
      setOutput(JSON.stringify(compiled, null, 2));
    } catch (error) {
      console.log(error);
      console.error("Execution error:", error);
      const errorMessage =
        error instanceof Error ? error.message : String(error);
      setOutput(`❌ Error:\n\n${errorMessage}`);
    } finally {
      setIsLoading(false);
    }
  }, [code]);

  return (
    <div className="ideContainer">
      <header className="header">
        <h1>₿ithoven Web IDE 🎼 💻 </h1>

        <div className="header-controls">
          <span className="onlineBadge">Online</span>

          {/* GitHub Link Start */}
          <a
            href="https://github.com/ChrisCho-H/bithoven"
            target="_blank"
            rel="noopener noreferrer"
            className="githubLink"
            aria-label="View source on GitHub"
          >
            <svg
              viewBox="0 0 24 24"
              width="24"
              height="24"
              stroke="currentColor"
              strokeWidth="2"
              fill="none"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
            </svg>
            <span>GitHub</span>
          </a>
          {/* GitHub Link End */}
        </div>
      </header>

      <main>
        <div className="editorSection">
          <label htmlFor="codemirror-editor">Code Editor (pretty)</label>
          <CodeMirror
            id="codemirror-editor"
            value={code}
            height="300px"
            theme="dark"
            extensions={[customKeywordHighlighting]}
            onChange={(value) => setCode(value)}
          />
        </div>

        <button
          className="compileButton"
          onClick={handleCompile}
          disabled={isLoading}
        >
          {isLoading ? "Compiling..." : "Compile & Run"}
        </button>

        <div className="outputSection">
          <label htmlFor="output">Output</label>
          <pre id="output" className="output">
            {output}
          </pre>
        </div>
      </main>
    </div>
  );
}

export default App;
