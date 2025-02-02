import { JSX } from "preact";
import { useState } from "preact/hooks";

export default function LoginForm() {
  const [username, setUsername] = useState<string>("");
  const [password, setPassword] = useState<string>("");

  const handleSubmit = (e: JSX.TargetedEvent<HTMLFormElement>) => {
    e.preventDefault();

    console.log("Username:", username);
    console.log("Password:", password);

    // ここでログイン処理などを実行
  };

  return (
    <form
      onSubmit={handleSubmit}
      style={{ display: "flex", flexDirection: "column", gap: "8px" }}
    >
      <div>
        <input
          style={{ width: "300px", height: "20px", padding: "4px", border: "1px solid #ccc", borderRadius: "4px" }}
          placeholder="LOGIN ID"
          type="text"
          id="username"
          value={username}
          onInput={(e: JSX.TargetedEvent<HTMLInputElement>) => setUsername(e.currentTarget.value)}
          required // Added required attribute
        />
      </div>
      <div>
        <input
          style={{ width: "300px", height: "20px", padding: "4px", border: "1px solid #ccc", borderRadius: "4px" }}
          placeholder="PASSWORD"
          type="password"
          id="password"
          value={password}
          onInput={(e: JSX.TargetedEvent<HTMLInputElement>) => setPassword(e.currentTarget.value)}
          required // Added required attribute
        />
      </div>
      <button
        style={{ width: "300px", height: "30px", padding: "4px", border: "none", borderRadius: "4px", backgroundColor: "#007bff", color: "white", cursor: "pointer" }}
        type="submit"
      >
        ログイン
      </button>
    </form>
  );
}