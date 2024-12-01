import LoginForm from "../../islands/LoginForm.tsx";

export default function Login() {
    return (
        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh;">
            <h1 style="font-size: 48px;">COSAN</h1>
            <p style="font-size: 24px; font-weight: bold; margin-bottom: 24px;">「あなたの古参歴、唯一の証」</p>
            <p></p>
            <LoginForm />
        </div>
    )
}