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
            padding: "10px",
            textAlign: "center",
            fontFamily: "hiragino kaku gothic pro, sans-serif",
            }}
        >
            <Header />

            <SearchWord />

            <MenuBar />
        </div>
    )
}