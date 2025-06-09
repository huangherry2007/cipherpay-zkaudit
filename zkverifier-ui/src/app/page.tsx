"use client";
import React, { useState, useRef, ChangeEvent, FormEvent, useEffect } from "react";
import Image from "next/image";
import styles from "./page.module.css";

const Page = () => {
  const [proofText, setProofText] = useState("");
  const [publicInputsText, setPublicInputsText] = useState("");
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [history, setHistory] = useState<Array<{ proof: string; publicInputs: string; result: string; timestamp: number }>>([]);
  const [isDarkMode, setIsDarkMode] = useState(false);
  const proofFileRef = useRef<HTMLInputElement>(null);
  const publicInputsFileRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const savedHistory = localStorage.getItem("verificationHistory");
    if (savedHistory) {
      setHistory(JSON.parse(savedHistory));
    }
  }, []);

  const handleFileChange = async (
    e: ChangeEvent<HTMLInputElement>,
    setter: (v: string) => void
  ) => {
    const file = e.target.files?.[0];
    if (!file) return;
    const text = await file.text();
    setter(text);
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
  };

  const handleDrop = async (e: React.DragEvent, setter: (v: string) => void) => {
    e.preventDefault();
    e.stopPropagation();
    const file = e.dataTransfer.files?.[0];
    if (!file) return;
    const text = await file.text();
    setter(text);
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setResult(null);
    setError(null);
    setLoading(true);
    try {
      const res = await fetch("/api/verify", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ proof: proofText, publicInputs: publicInputsText }),
      });
      const data = await res.json();
      if (res.ok) {
        setResult(JSON.stringify(data, null, 2));
        const newHistory = [...history, { proof: proofText, publicInputs: publicInputsText, result: JSON.stringify(data, null, 2), timestamp: Date.now() }];
        setHistory(newHistory);
        localStorage.setItem("verificationHistory", JSON.stringify(newHistory));
      } else {
        setError(data.error || "Verification failed");
      }
    } catch (err: any) {
      setError(err.message || "Unknown error");
    } finally {
      setLoading(false);
    }
  };

  const toggleTheme = () => {
    setIsDarkMode(!isDarkMode);
  };

  return (
    <div className={`${styles.page} ${isDarkMode ? styles.dark : ""}`}>
      <main className={styles.main}>
        <Image
          className={styles.logo}
          src="/next.svg"
          alt="Next.js logo"
          width={180}
          height={38}
          priority
        />
        <form
          className="w-full max-w-lg bg-white rounded shadow p-6 flex flex-col gap-4"
          onSubmit={handleSubmit}
          aria-label="Proof Verification Form"
        >
          <h1 className="text-2xl font-bold mb-2">Zero-Knowledge Proof Verifier</h1>
          <label className="block font-semibold" htmlFor="proofFile">
            Upload Proof File
          </label>
          <input
            id="proofFile"
            type="file"
            accept=".txt,.json"
            ref={proofFileRef}
            className="mb-2 border rounded p-2"
            onChange={e => handleFileChange(e, setProofText)}
            aria-label="Upload proof file"
          />
          <label className="block font-semibold" htmlFor="proofText">
            Or Paste Proof
          </label>
          <textarea
            id="proofText"
            className="w-full border rounded p-2 mb-2 min-h-[60px]"
            value={proofText}
            onChange={e => setProofText(e.target.value)}
            aria-label="Paste proof"
          />
          <label className="block font-semibold" htmlFor="publicInputsFile">
            Upload Public Inputs File
          </label>
          <input
            id="publicInputsFile"
            type="file"
            accept=".txt,.json"
            ref={publicInputsFileRef}
            className="mb-2 border rounded p-2"
            onChange={e => handleFileChange(e, setPublicInputsText)}
            aria-label="Upload public inputs file"
          />
          <label className="block font-semibold" htmlFor="publicInputsText">
            Or Paste Public Inputs
          </label>
          <textarea
            id="publicInputsText"
            className="w-full border rounded p-2 mb-2 min-h-[60px]"
            value={publicInputsText}
            onChange={e => setPublicInputsText(e.target.value)}
            aria-label="Paste public inputs"
          />
          <button
            type="submit"
            className="bg-blue-600 text-white font-semibold py-2 px-4 rounded hover:bg-blue-700 disabled:opacity-50"
            disabled={loading}
            aria-label="Submit proof for verification"
          >
            {loading ? "Verifying..." : "Verify Proof"}
          </button>
          {result && (
            <pre className="bg-green-100 text-green-800 rounded p-2 mt-2 whitespace-pre-wrap break-all">
              {result}
            </pre>
          )}
          {error && (
            <div className="bg-red-100 text-red-800 rounded p-2 mt-2">{error}</div>
          )}
        </form>
        <div className="mt-4">
          <h2 className="text-xl font-bold mb-2">Verification History</h2>
          <ul className="space-y-2">
            {history.map((item, index) => (
              <li key={index} className="bg-gray-100 p-2 rounded">
                <p>Proof: {item.proof.substring(0, 50)}...</p>
                <p>Public Inputs: {item.publicInputs.substring(0, 50)}...</p>
                <p>Result: {item.result}</p>
                <p>Time: {new Date(item.timestamp).toLocaleString()}</p>
              </li>
            ))}
          </ul>
        </div>
        <button
          onClick={toggleTheme}
          className="mt-4 bg-gray-200 text-gray-800 font-semibold py-2 px-4 rounded hover:bg-gray-300"
          aria-label="Toggle dark mode"
        >
          {isDarkMode ? "Light Mode" : "Dark Mode"}
        </button>
      </main>
      <footer className={styles.footer}>
        <a
          href="https://nextjs.org/learn?utm_source=create-next-app&utm_medium=appdir-template&utm_campaign=create-next-app"
          target="_blank"
          rel="noopener noreferrer"
        >
          <Image
            aria-hidden
            src="/file.svg"
            alt="File icon"
            width={16}
            height={16}
          />
          Learn
        </a>
        <a
          href="https://vercel.com/templates?framework=next.js&utm_source=create-next-app&utm_medium=appdir-template&utm_campaign=create-next-app"
          target="_blank"
          rel="noopener noreferrer"
        >
          <Image
            aria-hidden
            src="/window.svg"
            alt="Window icon"
            width={16}
            height={16}
          />
          Examples
        </a>
        <a
          href="https://nextjs.org?utm_source=create-next-app&utm_medium=appdir-template&utm_campaign=create-next-app"
          target="_blank"
          rel="noopener noreferrer"
        >
          <Image
            aria-hidden
            src="/globe.svg"
            alt="Globe icon"
            width={16}
            height={16}
          />
          Go to nextjs.org →
        </a>
      </footer>
    </div>
  );
};

export default Page;
