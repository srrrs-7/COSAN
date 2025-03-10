import { JSX } from "preact";
import { useState } from "preact/hooks";

export default function RegisterForm() {
  const [mailAddress, setMailAddress] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [checkPassword, setCheckPassword] = useState<string>("");

  const handleSubmit = (e: JSX.TargetedEvent<HTMLFormElement>) => {
    e.preventDefault();

    console.log("Mail Address:", mailAddress);
    console.log("Password:", password);
    console.log("Check Password:", checkPassword);

    // ここで登録処理などを実行
  };

  return (
    <form
      onSubmit={handleSubmit}
      style={{ display: "flex", flexDirection: "column", gap: "8px" }}
    >

      <div>
        <input
          style={{ width: "300px", height: "20px", padding: "4px", border: "1px solid #ccc", borderRadius: "10px" }}
          placeholder="メールアドレス"
          type="email"
          id="mail_address"
          value={mailAddress}
          onInput={(e: JSX.TargetedEvent<HTMLInputElement>) => setMailAddress(e.currentTarget.value)}
          required
        />
      </div>

      <div>
        <input
          style={{ width: "300px", height: "20px", padding: "4px", border: "1px solid #ccc", borderRadius: "10px" }}
          placeholder="PASSWORD"
          type="password"
          id="password"
          value={password}
          onInput={(e: JSX.TargetedEvent<HTMLInputElement>) => setPassword(e.currentTarget.value)}
          required
          minLength={8}
        />
      </div>

      <div>
        <input
          style={{ width: "300px", height: "20px", padding: "4px", border: "1px solid #ccc", borderRadius: "10px" }}
          placeholder="CHECK PASSWORD"
          type="password"
          id="check_password"
          value={checkPassword}
          onInput={(e: JSX.TargetedEvent<HTMLInputElement>) => setCheckPassword(e.currentTarget.value)}
          required
          minLength={8}
        />
      </div>

      <button
        style={{ 
          width: "100px", 
          height: "30px", 
          padding: "2px", 
          border: "none", 
          backgroundColor: "#6aac98", 
          cursor: "pointer", 
          alignSelf: "center",
          borderRadius: "10px",
        }}
        type="submit"
        >
        アカウント登録
      </button>
    </form>
  );
}