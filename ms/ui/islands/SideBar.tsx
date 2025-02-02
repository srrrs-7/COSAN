import { JSX } from "preact";
import { useState, useEffect } from "preact/hooks";

interface Props {
  isOpen: boolean;
}
  
export default function SideBar({ isOpen }: Props) {
  const [searchPath, setSearchPath] = useState<string>("");
  const [registerPath, setRegisterPath] = useState<string>("");

  useEffect(() => {
    const path = window.location.pathname;
    const deviceType = path.split("/")[1];
    setSearchPath(`/${deviceType}/mypage/search`);
    setRegisterPath(`/${deviceType}/mypage/register`);
  }, []);

  return (
    <aside
      style={{
        width: "10%",
        height: "100%",
        position: "fixed",
        top: 0,
        left: 0,
        marginTop: "4rem",
        backgroundColor: "#4a5568",
        color: "white",
        transition: "transform 0.3s ease-in-out",
        transform: isOpen ? "translateX(0)" : "translateX(-100%)",
        display: "block",
      }}
    >
      <div style={{ padding: "1rem", marginTop: "4rem" }}>
        <h2 style={{ fontSize: "1.25rem", fontWeight: "bold" }}>
          Menu
        </h2>
        <ul style={{listStyle: "none", padding: 0}}>
          <li style={{ marginTop: "1rem" }}>
            <a href={searchPath} style={{ color: "white", textDecoration: "none" }}>
              検索
            </a>
          </li>
          <li style={{ marginTop: "1rem" }}>
            <a href={registerPath} style={{ color: "white", textDecoration: "none" }}>
              登録
            </a>
          </li>
          <li style={{ marginTop: "1rem" }}>
            <a href="#" style={{ color: "white", textDecoration: "none" }}>
              コレクション
            </a>
          </li>
          <li style={{ marginTop: "1rem" }}>
            <a href="#" style={{ color: "white", textDecoration: "none" }}>
              コミュニティ
            </a>
          </li>
          <li style={{ marginTop: "1rem" }}>
            <a href="#" style={{ color: "white", textDecoration: "none" }}>
              ログイン
            </a>
          </li>
        </ul>
      </div>
      {/* Media query handling for larger screens */}
      {(typeof window !== "undefined" && window.innerWidth >= 768) && (
        <style jsx global>{`
          aside {
            display: none;
          }
        `}</style>
      )}
    </aside>
  );
}