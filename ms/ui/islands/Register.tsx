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
      <style>{`
        .container {
          display: flex;
          flex-direction: column;
          align-items: center;
          justify-content: center;
          height: 100vh;
        }

        .title {
          font-size: 48px;
          margin-bottom: 0;
        }

        .subtitle {
          font-size: 24px;
          font-weight: bold;
          margin-bottom: 24px;
          margin-top: 8px;
        }

        .link {
          margin-top: 24px;
        }
      `}</style>
      {/*  Rest of the component is same as before */}
      <h1 className="title">COSAN</h1>
      <p className="subtitle">登録して古参になりましょう</p>
      <RegisterForm />
      <Agreement isAgreed={isAgreed} />
      <div className="link">
        <a href="/mb">ログイン画面へ戻る</a>
      </div>
    </div>
  );
}