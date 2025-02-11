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
            padding: "10px", 
            textAlign: "center", 
            fontFamily: "hiragino kaku gothic pro, sans-serif",
            }}
        >
            <Header />
            
            <p style={{ fontSize: "20px" }}>
                古参を証明する唯一の証
            </p>
            <p style={{ marginBottom: "8px" }}>
                あなたの推しを登録しましょう。
            </p>
            <p style={{ marginBottom: "8px" }}>
                あなたの推しを登録することで、古参であることを証明できます。
            </p>
            
            <RegisterWord />

            <MenuBar />
        </div>
    )
}