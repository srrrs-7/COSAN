import RegisterForm from "../../islands/RegisterForm.tsx";
import Agreement from "../../islands/Agreement.tsx";

export default function Register() {
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
      <Agreement />
      <div className="link">
        <a href="/mb">ログイン画面へ戻る</a>
      </div>
    </div>
  );
}