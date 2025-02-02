import { JSX } from "preact";
import { useState } from "preact/hooks";
import Sidebar from "../islands/Sidebar.tsx";
import RegisterWord from "../islands/mypage/RegisterWord.tsx";

export default function MyPage() {
  const [isOpen, setIsOpen] = useState<boolean>(false);

  const toggleSidebar = () => {
    setIsOpen(!isOpen);
  };

  return (
    <div>
      <button
        onClick={toggleSidebar}
        style={{
          position: "fixed",
          top: "1rem",
          left: "1rem",
          backgroundColor: "#4299e1",
          ":hover": {
            backgroundColor: "#2b6cb0",
          },
          color: "white",
          fontWeight: "bold",
          padding: "0.5rem 1rem",
          borderRadius: "0.25rem",
          border: "none",
          cursor: "pointer",
        }}
      >
        {isOpen ? "close" : "open"}
      </button>
      <Sidebar isOpen={isOpen} />
      <div style={{ display: "flex", justifyContent: "center", alignItems: "center", height: "100vh" }}>
        <RegisterWord />
      </div>
    </div>
  );
}