import LoginForm from "../../islands/LoginForm.tsx";

export default function Login() {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        height: "100vh",
        padding: "20px", // Add padding for better spacing
        textAlign: "center", // Center align text
        fontFamily: "'Helvetica Neue', Arial, sans-serif", // Set a default font family
      }}
    >
      <h1 style={{ fontSize: "48px", marginBottom: "16px" }}>COSAN</h1>
      <p style={{ fontSize: "24px", fontWeight: "bold", marginBottom: "24px" }}>
        「あなたの古参歴、唯一の証」
      </p>
      <p style={{ marginBottom: "8px" }}>
        COSAN は、「古参歴」を登録できるサイトです!
      </p>
      <p style={{ marginBottom: "8px" }}>
        新たな推しの第一発見者になり、登録しましょう。
      </p>
      <p style={{ marginBottom: "8px" }}>推しとは多岐に渡るでしょう。</p>
      <p style={{ marginBottom: "8px" }}>アイドル?スポーツ選手? 身の回りの人?</p>
      <p style={{ marginBottom: "8px" }}>それは人ではないかもしれません。</p>
      <p style={{ marginBottom: "24px" }}>新たにできたお店?</p>
      <LoginForm />
      <div style={{ marginTop: "24px" }}>
        <a href="/mb/register" style={{ color: "#007bff", textDecoration: "none" }}>
          無料新規登録はこちら
        </a>
      </div>
    </div>
  );
}