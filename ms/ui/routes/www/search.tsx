import Header from "../../islands/common/Header.tsx";
import SearchWord from "../../islands/mypage/SearchWord.tsx";
import MenuBar from "../../islands/common/MenuBar.tsx";

export default function search() {
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

            <SearchWord />

            <MenuBar />
        </div>
    )
}