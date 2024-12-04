import LoginForm from "../../islands/LoginForm.tsx";

export default function Login() {
    return (
        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh;">
            <h1 style="font-size: 48px;">COSAN</h1>
            <p style="font-size: 24px; font-weight: bold; margin-bottom: 24px;">「あなたの古参歴、唯一の証」</p>
            <p>「あなたの古参歴、唯一の証。」</p>
            <p>COSAN は、「古参歴」を登録できるサイトです!</p>
            <p>新たな推しの第一発見者になり、登録しましょう。</p>
            <p>推しとは多岐に渡るでしょう。</p>
            <p>アイドル?スポーツ選手? 身の回りの人?</p>
            <p>それは人ではないかもしれません。</p>
            <p style="margin-bottom: 24px;">新たにできたお店?</p>
            <LoginForm />
            <div style="margin-top: 24px;">
                <a href="">無料新規登録はこちら</a>
            </div>
        </div>
    )
}