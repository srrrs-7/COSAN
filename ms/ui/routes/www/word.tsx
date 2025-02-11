import Header from "../../islands/common/Header.tsx";
import RegisterWord from "../../islands//mypage/RegisterWord.tsx";
import MenuBar from "../../islands/common/MenuBar.tsx";

export default function word() {
    return (
        <div
            style={{
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            justifyContent: "center",
            padding: "10px", // Add padding for better spacing
            textAlign: "center", // Center align text
            fontFamily: "hiragino kaku gothic pro, sans-serif", // Set a default font family
            }}
        >
            <Header />
            
            <div>
                <div style={{ display: "flex", flexDirection: "column", alignItems: "center", justifyContent: "center", marginBottom: "16px" }}>
                <p style={{ fontSize: "20px" }}>
                    古参を証明する唯一の証
                </p>
                <p style={{ marginBottom: "8px" }}>
                    あなたの推しを登録しましょう。
                </p>
                <p style={{ marginBottom: "8px" }}>
                    あなたの推しを登録することで、古参であることを証明できます。
                </p>
                </div>

                <div style={{ display: "flex", justifyContent: "center", alignItems: "center" }}>
                    <RegisterWord />
                </div>

                <MenuBar />
            </div>
        </div>
    )
}