import RegisterForm from "../../islands/RegisterForm.tsx";

export default function Register() {
    return (
        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh;">
            <h1 style="font-size: 48px;">COSAN</h1>
            <p style="font-size: 24px; font-weight: bold; margin-bottom: 24px;">登録して古参になりましょう</p>
            <p>
                <a href="/www/agreement">利用規約</a>とプライバシーポリシーへの同意が必要です
            </p>
            <div style="display: flex; flex-direction: row; gap: 8px; margin-bottom: 24px;">
                <input type="checkbox" />
                <p>利用規約とプライバシーポリシーに同意する</p>
            </div>
            <RegisterForm />
        </div>
    )
}