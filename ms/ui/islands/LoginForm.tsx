import { JSX } from "preact";
import { useState } from "preact/hooks";

export default function LoginForm() {
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
  
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
                    placeholder="LOGIN ID"
                    type="text"
                    id="username"
                    value={username}
                    onInput={(e) => setUsername(e.target.value)}
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
        <button
            style="width: 300px; height: 30px;"
            type="submit"
        >
            ログイン
        </button>
      </form>
    );
  }