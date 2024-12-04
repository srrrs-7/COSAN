import { JSX } from "preact";
import { useState } from "preact/hooks";

export default function RegisterForm() {
    const [mailAddress, setMailAddress] = useState('');
    const [password, setPassword] = useState('');
    const [checkPassword, setCheckPassword] = useState('');
  
    const handleSubmit = (e) => {
        e.preventDefault()
  
        console.log('Username:', username);
        console.log('Password:', password);
    };
  
    return (
        <form 
            onSubmit={handleSubmit} 
            style="display: flex; flex-direction: column; gap: 8px;"
        >
            <div>
                <input
                    style="width: 300px; height: 20px;"
                    placeholder="メールアドレス"
                    type="text"
                    id="mail_address"
                    value={mailAddress}
                    onInput={(e) => setMailAddress(e.target.value)}
                />
            </div>
            <div>
                <input
                    style="width: 300px; height: 20px;"
                    placeholder="PASSWORD"
                    type="password"
                    id="password"
                    value={password}
                    onInput={(e) => setPassword(e.target.value)}
                />
            </div>
            <div>
                <input
                    style="width: 300px; height: 20px;"
                    placeholder="CHECK PASSWORD"
                    type="password"
                    id="check_password"
                    value={checkPassword}
                    onInput={(e) => setCheckPassword(e.target.value)}
                />
            </div>
        <button
            style="width: 300px; height: 30px;"
            type="submit"
        >
            アカウント登録
        </button>
      </form>
    );
  }