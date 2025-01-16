import { JSX } from "preact";
import { useState } from "preact/hooks";

export default function RegisterForm() {
  const [mailAddress, setMailAddress] = useState<string>('');
  const [password, setPassword] = useState<string>('');
  const [checkPassword, setCheckPassword] = useState<string>('');

  const handleSubmit = (e: JSX.TargetedEvent<HTMLFormElement>) => {
    e.preventDefault();

    console.log('Mail Address:', mailAddress);
    console.log('Password:', password);
    console.log('Check Password:', checkPassword);

    // ここで登録処理などを実行
  };

  return (
    <form
      onSubmit={handleSubmit}
      style={{ display: "flex", flexDirection: "column", gap: "8px" }}
    >
      <div>
        <input
          style={{ width: "300px", height: "20px", padding: "4px", border: "1px solid #ccc", borderRadius: "4px" }}
          placeholder="メールアドレス"
          type="email" // Changed type to email
          id="mail_address"
          value={mailAddress}
          onInput={(e: JSX.TargetedEvent<HTMLInputElement>) => setMailAddress(e.currentTarget.value)}
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
          minLength={8} // Added minLength attribute
        />
      </div>
      <div>
        <input
          style={{ width: "300px", height: "20px", padding: "4px", border: "1px solid #ccc", borderRadius: "4px" }}
          placeholder="CHECK PASSWORD"
          type="password"
          id="check_password"
          value={checkPassword}
          onInput={(e: JSX.TargetedEvent<HTMLInputElement>) => setCheckPassword(e.currentTarget.value)}
          required // Added required attribute
          minLength={8} // Added minLength attribute
        />
      </div>
      <button
        style={{ width: "300px", height: "30px", padding: "4px", border: "none", borderRadius: "4px", backgroundColor: "#007bff", color: "white", cursor: "pointer" }}
        type="submit"
      >
        アカウント登録
      </button>
    </form>
  );
}