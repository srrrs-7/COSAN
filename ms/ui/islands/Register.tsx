import RegisterForm from "../islands/register/RegisterForm.tsx";
import Agreement from "../components/Agreement.tsx";
import { useState } from "preact/hooks";

export default function Register() {
  const [isAgreed, setIsAgreed] = useState<boolean>(false);
  
  const checkAgreed = () => {
    setIsOpen(!isOpen);
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', height: '100vh' }}>
      <h1 style={{ fontSize: "48px" }}>COSAN</h1>
      <p style={{ fontSize: "24px", fontWeight: "bold", marginBottom: "24px", marginTop: "8px" }}>登録して古参になりましょう</p>
      <Agreement isAgreed={isAgreed} />
      <input
        style={{ marginBottom: "24px" }} 
        type="checkbox" 
        checked={isAgreed} 
        onChange={checkAgreed} 
      />
      <RegisterForm />
      <div style={{ marginTop: "24px" }} >
        <a href="/mb">ログイン画面へ戻る</a>
      </div>
    </div>
  );
}