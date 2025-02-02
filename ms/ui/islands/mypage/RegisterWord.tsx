import { JSX } from "preact";
import { useState } from "preact/hooks";

export default function RegisterWord() {
  const [word, setWord] = useState<string>("");

  const handleSubmit = (e: JSX.TargetedEvent<HTMLFormElement>) => {
    e.preventDefault();
    
    console.log("Word:", word);

    // ここで登録処理などを実行
  };

  return (
    <div style={{ display: "flex", flexDirection: "column", alignItems: "center", justifyContent: "center", width: "500px", height: "500px", color: "white" }}>
      <form
        onSubmit={handleSubmit}
        style={{ display: "flex", flexDirection: "column", gap: "8px" }}
      >
        <div>
          <input
            style={{ width: "300px", height: "20px", padding: "4px", border: "1px solid #ccc", borderRadius: "4px" }}
            placeholder="登録文言"
            type="text"
            id="word"
            value={word}
            onInput={(e: JSX.TargetedEvent<HTMLInputElement>) => setWord(e.currentTarget.value)}
            required
          />
        </div>
        <button
          style={{ width: "300px", height: "30px", padding: "4px", border: "none", borderRadius: "4px", backgroundColor: "#007bff", color: "white", cursor: "pointer" }}
          type="submit"
        >
          文言を登録
        </button>
      </form>
    </div>
  );
}